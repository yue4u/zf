use gdnative::{
    api::{AnimationPlayer, AudioStreamPlayer, ShaderMaterial, TextureRect},
    prelude::*,
};

use crate::refs::path::LevelName;

#[derive(NativeClass)]
#[inherit(TextureRect)]
pub struct ScreenTransition {
    base: Ref<TextureRect>,
    animation_player: Option<Ref<AnimationPlayer>>,
    audio_stream_player: Option<Ref<AudioStreamPlayer>>,
    next_level: Option<LevelName>,
}

#[methods]
impl ScreenTransition {
    fn new(base: TRef<TextureRect>) -> Self {
        ScreenTransition {
            base: base.claim(),
            animation_player: None,
            audio_stream_player: None,
            next_level: None,
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

        let audio_stream_player = unsafe {
            base.get_node_as::<AudioStreamPlayer>("./ScreenTransitionAudioStreamPlayer")
                .unwrap()
        };

        animation_player
            .connect(
                "animation_finished",
                base,
                "on_animation_finished",
                VariantArray::new_shared(),
                0,
            )
            .unwrap();

        self.animation_player = Some(animation_player.claim());
        self.audio_stream_player = Some(audio_stream_player.claim());

        Some(())
    }

    #[method]
    /// Start playing transition and set next scene target but not start right now
    pub fn to(&mut self, next_scene: LevelName) {
        self.next_level = Some(next_scene);
        let audio_stream_player = unsafe { self.audio_stream_player.unwrap().assume_safe() };
        let animation_player = unsafe { self.animation_player.unwrap().assume_safe() };

        audio_stream_player.play(0.);
        animation_player.play("Pixelate", -1., 1.0, false);
    }

    #[method]
    /// Actualy changed current scene. This will be called in the middle of transition.
    pub fn change_scene(&mut self) {
        let tree = unsafe { self.base.assume_safe().get_tree().unwrap().assume_safe() };
        let level = self.next_level.take().unwrap();
        tree.change_scene(level.path()).unwrap();
        tree.set_pause(false);
    }

    #[method]
    fn on_animation_finished(&self, #[base] _base: TRef<TextureRect>, _name: String) -> Option<()> {
        let audio_stream_player = unsafe { self.audio_stream_player?.assume_safe() };
        audio_stream_player.stop();
        Some(())
    }
}
