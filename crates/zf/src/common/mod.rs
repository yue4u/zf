pub mod logging;

use gdnative::{api::*, prelude::*};

use crate::{
    refs::{path::LevelName, HasPath},
    units::Player,
};

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

pub struct SceneLoader;

pub type PackedSceneRef = Ref<PackedScene, Unique>;

impl SceneLoader {
    pub fn load(path: &str) -> Option<PackedSceneRef> {
        let res = ResourceLoader::godot_singleton().load(path, "PackedScene", false)?;
        let scene = unsafe { res.assume_unique() }.cast::<PackedScene>()?;
        Some(scene)
    }

    pub fn instance_as<T>(scene: &PackedSceneRef) -> Option<Ref<T, Unique>>
    where
        T: GodotObject<Memory = ManuallyManaged> + SubClass<Node>,
    {
        let instance = scene.instance(0)?;
        let instance = unsafe { instance.assume_unique() };
        instance.cast::<T>()
    }

    pub fn load_and_instance_as<T>(path: &str) -> Option<Ref<T, Unique>>
    where
        T: GodotObject<Memory = ManuallyManaged> + SubClass<Node>,
    {
        let scene = SceneLoader::load(path)?;
        SceneLoader::instance_as::<T>(&scene)
    }
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
        let transform = unsafe {
            self.get_node(Player::path_from(&self))
                .unwrap()
                .assume_safe()
        }
        .cast::<Spatial>()?
        .global_transform();
        self.look_at(transform.origin, Vector3::UP);
        Some(())
    }
}

pub fn get_tree<'a>(base: &'a Node) -> TRef<'a, SceneTree> {
    unsafe { base.get_tree().unwrap().assume_safe() }
}

pub fn current_level<'a>(base: &'a Node) -> LevelName {
    let current_scene = get_tree(base).current_scene();
    match current_scene {
        Some(scene) => {
            let path = unsafe { scene.assume_safe() }.filename();
            LevelName::from_path(path.to_string().as_str())
        }
        None => LevelName::Unknown,
    }
}

pub enum StyledLabel {
    Code,
    Hint,
}

impl StyledLabel {
    pub fn paint(&self, input: impl ToString) -> String {
        use nu_ansi_term::Color::*;
        match self {
            StyledLabel::Code => Rgb(0, 0, 0).on(Rgb(255, 194, 60)),
            StyledLabel::Hint => Rgb(0, 0, 0).on(LightGreen),
        }
        .paint(format!(" {} ", input.to_string()))
        .to_string()
    }
}
