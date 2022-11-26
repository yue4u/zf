use wasmtime::{AsContextMut, Caller, Instance, Memory, Store, StoreContext};
use zf_bridge::{config, decode_from_slice, Tag};

use crate::runtime::ExtendedStore;

pub fn write_string_with_caller<T>(caller: &mut Caller<'_, T>, string: String) -> i64 {
    let content = string.as_bytes();
    let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
    let alloc_string = caller
        .get_export("alloc_string")
        .unwrap()
        .into_func()
        .unwrap();

    let mut store = caller.as_context_mut();
    let len = content.len() as i32;
    let ptr = alloc_string
        .typed::<i32, i32, _>(&mut store)
        .unwrap()
        .call(&mut store, len)
        .unwrap();

    memory.write(&mut store, ptr as usize, content).unwrap();

    debug_assert_eq!(
        &memory.data(&store)[ptr as usize..ptr as usize + len as usize],
        content
    );
    (ptr as i64) << 32 | (len as i64)
}

#[must_use]
pub(crate) fn write_string_from_host<T>(
    instance: Instance,
    store: &mut Store<T>,
    memory: &Memory,
    string: String,
) -> i64 {
    let content = string.as_bytes();
    let alloc_string = instance.get_func(&mut *store, "alloc_string").unwrap();

    let len = content.len() as i32;
    let ptr = alloc_string
        .typed::<i32, i32, _>(&mut *store)
        .unwrap()
        .call(&mut *store, len)
        .unwrap();

    memory.write(&mut *store, ptr as usize, content).unwrap();

    debug_assert_eq!(
        &memory.data(&&mut *store)[ptr as usize..ptr as usize + len as usize],
        content
    );
    Tag::into(ptr, len)
}
pub fn decode_from_host<'a, T: 'a, D: zf_bridge::de::Decode>(
    store: impl Into<StoreContext<'a, T>>,
    memory: &Memory,
    tag: i64,
) -> D {
    let (ptr, len) = Tag::from(tag);

    let data = &memory.data(store)[ptr as usize..ptr as usize + len as usize];
    let (args, _) = decode_from_slice(data, config::standard()).unwrap();
    args
}

pub fn decode_from_caller<T, D: zf_bridge::de::Decode>(
    caller: &mut Caller<'_, ExtendedStore<T>>,
    tag: i64,
) -> D {
    let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
    let mut store = caller.as_context_mut();

    decode_from_host(&mut store, &memory, tag)
}
