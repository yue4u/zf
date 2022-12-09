use gdnative::{
    api::{object::ConnectFlags, AnimationPlayer, ImageTexture, TextureRect},
    prelude::*,
};

#[derive(NativeClass)]
#[inherit(TextureRect)]
pub struct ScreenTransition {
    #[property(default = true)]
    pub enter: bool,
    base: Ref<TextureRect>,
    animation_player: Option<Ref<AnimationPlayer>>,
}

#[methods]
impl ScreenTransition {
    fn new(base: TRef<TextureRect>) -> Self {
        ScreenTransition {
            base: base.claim(),
            enter: true,
            animation_player: None,
        }
    }

    #[method]
    fn _ready(&mut self, #[base] base: TRef<TextureRect>) -> Option<()> {
        let animation_player = unsafe {
            base.get_node_as::<AnimationPlayer>("AnimationPlayer")
                .unwrap()
        };

        self.animation_player = Some(animation_player.claim());

        if self.enter {
            self.play(true)?;
        } else {
            base.set_visible(false);
        }

        base.connect(
            "tree_exiting",
            base,
            "on_tree_exiting",
            VariantArray::new_shared(),
            0,
        )
        .expect("failed to connect tree_exiting");

        Some(())
    }

    fn play(&self, backwards: bool) -> Option<()> {
        let base = unsafe { self.base.assume_safe() };
        let img = unsafe {
            base.get_viewport()?
                .assume_safe()
                .get_texture()?
                .assume_safe()
                .get_data()?
        };
        unsafe { img.assume_safe() }.flip_y();
        let capture = ImageTexture::new();
        capture.create_from_image(img, 7);
        base.set_texture(capture);

        let animation_player = unsafe { self.animation_player.unwrap().assume_safe() };

        animation_player
            .connect(
                "animation_finished",
                base,
                "animation_finished",
                VariantArray::new_shared(),
                ConnectFlags::ONESHOT.into(),
            )
            .unwrap();
        if !backwards {
            animation_player.play("Pixelate", -1., 1.0, true);
        } else {
            animation_player.play_backwards("Pixelate", -1.);
        }

        Some(())
    }

    #[method]
    fn animation_finished(&self, #[base] base: TRef<TextureRect>) -> Option<()> {
        godot_dbg!("animation_finished");
        base.set_visible(false);
        let animation_player = unsafe { self.animation_player.unwrap().assume_safe() };
        animation_player.disconnect("animation_finished", base, "animation_finished");
        Some(())
    }

    #[method]
    fn on_tree_exiting(&self, #[base] base: TRef<TextureRect>) {
        base.set_visible(true);
        godot_dbg!("on_tree_exiting");
        self.play(false);
    }
}
