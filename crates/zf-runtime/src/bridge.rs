use wasmtime::{AsContextMut, Caller, Instance, Memory, Store};

pub(crate) fn write_string_inside<T>(mut caller: Caller<'_, T>, string: String) -> i64 {
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

pub fn read_string_outside<T>(store: &Store<T>, memory: &Memory, parts: i64) -> String {
    let len = parts as i32;
    let ptr = (parts >> 32) as i32;
    String::from_utf8_lossy(memory.data(store)[ptr as usize..ptr as usize + len as usize].into())
        .to_string()
}

#[must_use]
pub(crate) fn write_string_outside<T>(
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
    (ptr as i64) << 32 | (len as i64)
}
