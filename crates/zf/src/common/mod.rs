use crate::path::HasPath;
use gdnative::{api::*, prelude::*};

pub type Id = u32;
pub type Position = Vector3;
pub type Rotation = Vector3;

pub trait Vector3DisplayShort {
    fn display(&self) -> String;
}

impl Vector3DisplayShort for Vector3 {
    fn display(&self) -> String {
        format!("{:.2}, {:.2}, {:.2}", self.x, self.y, self.z)
    }
}

pub fn load_as<T: GodotObject<Memory = ManuallyManaged> + SubClass<Node>>(
    path: &str,
) -> Option<Ref<T, Unique>> {
    let res = ResourceLoader::godot_singleton().load(path, "PackedScene", false)?;
    let scene = unsafe { res.assume_thread_local() }.cast::<PackedScene>()?;
    let instance = scene.instance(PackedScene::GEN_EDIT_STATE_INSTANCE)?;
    let instance = unsafe { instance.assume_unique() };
    instance.cast::<T>()
}

pub fn find_ref<'a, S, R>(target: TRef<Node>) -> Option<TRef<'a, R>>
where
    S: HasPath,
    R: SubClass<Node>,
{
    unsafe { target.get_node(S::path())?.assume_safe() }.cast::<R>()
}
