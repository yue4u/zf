use crate::{
    entities::SceneName,
    refs::{path, HasPath},
    units::Player,
};
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

pub(crate) trait LookAtPlauer {
    fn try_look_at_player(&self) -> Option<()>;
}

impl LookAtPlauer for TRef<'_, Spatial> {
    fn try_look_at_player(&self) -> Option<()> {
        let transform = find_ref::<Player, Spatial>(unsafe { self.get_node(".")?.assume_safe() })?
            .global_transform();
        self.look_at(transform.origin, Vector3::UP);
        Some(())
    }
}

pub fn get_tree<'a>(base: &'a Node) -> TRef<'a, SceneTree> {
    unsafe { base.get_tree().unwrap().assume_safe() }
}

pub fn current_scene<'a>(base: &'a Node) -> SceneName {
    let current_scene = get_tree(base).current_scene();
    godot_dbg!(current_scene);
    match current_scene {
        Some(scene) => {
            let name = unsafe { scene.assume_safe() }.filename();
            godot_dbg!(&name);
            match name.to_string().as_str() {
                path::levels::START_MENU => SceneName::StartMenu,
                path::levels::SANDBOX => SceneName::Sandbox,
                path::levels::TUTORIAL_MOVEMENT => SceneName::Tutorial,
                _ => SceneName::Unknown,
            }
        }
        None => SceneName::Unknown,
    }
}
