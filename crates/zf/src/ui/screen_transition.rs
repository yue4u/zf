use gdnative::{
    api::{AnimationPlayer, ShaderMaterial, TextureRect},
    prelude::*,
};

use crate::refs::path::SceneName;

#[derive(NativeClass)]
#[inherit(TextureRect)]
pub struct ScreenTransition {
    base: Ref<TextureRect>,
    animation_player: Option<Ref<AnimationPlayer>>,
    next_scene: Option<&'static str>,
}

#[methods]
impl ScreenTransition {
    fn new(base: TRef<TextureRect>) -> Self {
        ScreenTransition {
            base: base.claim(),
            animation_player: None,
            next_scene: None,
        }
    }

    #[method]
    fn _ready(&mut self, #[base] base: TRef<TextureRect>) -> Option<()> {
        unsafe {
            base.material()
                .unwrap()
                .assume_safe()
                .cast::<ShaderMaterial>()
                .unwrap()
        }
        .set_shader_param("factor", 0.0);

        let animation_player = unsafe {
            base.get_node_as::<AnimationPlayer>("./ScreenTransitionPlayer")
                .unwrap()
        };

        // base.connect(
        //     "animation_finished",
        //     base,
        //     "on_animation_finished",
        //     VariantArray::new_shared(),
        //     0,
        // )
        // .unwrap();

        self.animation_player = Some(animation_player.claim());

        Some(())
    }

    #[method]
    /// Start playing transition and set next scene target but not start right now
    pub fn to(&mut self, next_scene: SceneName) {
        self.next_scene = Some(next_scene.path());
        let animation_player = unsafe { self.animation_player.unwrap().assume_safe() };
        animation_player.play("Pixelate", -1., 1.0, false);
    }

    #[method]
    /// Actualy changed current scene. This will be called in the middle of transition.
    pub fn change_scene(&mut self) {
        let tree = unsafe { self.base.assume_safe().get_tree().unwrap().assume_safe() };
        let path = self.next_scene.take().unwrap();
        tree.change_scene(path).unwrap();
        tree.set_pause(false);
    }

    // #[method]
    // fn on_animation_finished(
    //     &mut self,
    //     #[base] _base: TRef<AnimationPlayer>,
    //     _name: String,
    // ) -> Option<()> {
    //     tracing::debug!("{:?}","on_animation_finished");
    //
    //     Some(())
    // }
}
