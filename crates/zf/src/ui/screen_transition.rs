use gdnative::{
    api::{AnimationPlayer, ShaderMaterial, TextureRect},
    prelude::*,
};

use crate::refs::path::SceneName;

#[derive(NativeClass)]
#[inherit(TextureRect)]
pub struct ScreenTransition {
    // backwards_texture: Option<Ref<ImageTexture, Unique>>,
    base: Ref<TextureRect>,
    // middle: bool,
    animation_player: Option<Ref<AnimationPlayer>>,
    next_scene: Option<&'static str>,
}

// enum Directional {
//     Forwards,
//     Backwards,
// }

#[methods]
impl ScreenTransition {
    fn new(base: TRef<TextureRect>) -> Self {
        // skip show on init
        // base.set_visible(false);
        ScreenTransition {
            // backwards_texture: None,
            base: base.claim(),
            // middle: false,
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

        // let root = unsafe { base.get_node("/root").unwrap().assume_safe() };

        // root.connect(
        //     "child_entered_tree",
        //     base,
        //     "on_child_entered_tree",
        //     VariantArray::new_shared(),
        //     0,
        // )
        // .expect("failed to connect child_entered_tree");

        // root.connect(
        //     "child_exiting_tree",
        //     base,
        //     "on_child_exiting_tree",
        //     VariantArray::new_shared(),
        //     0,
        // )
        // .expect("failed to connect child_exiting_tree");

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
    pub fn play_transition(&mut self, next_scene: SceneName) {
        self.next_scene = Some(next_scene.path());
        let animation_player = unsafe { self.animation_player.unwrap().assume_safe() };
        // match direction {
        // Directional::Forwards => {
        animation_player.play("Pixelate", -1., 1.0, false);
        // }
        // Directional::Backwards => {
        //     animation_player.play_backwards("Pixelate", -1.);
        // }
        // };
        // Some(())
    }

    #[method]
    pub fn change_scene(&self) {
        let tree = unsafe { self.base.assume_safe().get_tree().unwrap().assume_safe() };
        let path = self.next_scene.unwrap();
        tree.change_scene(path).unwrap();
        tree.set_pause(false);
    }

    // #[method]
    // fn on_animation_finished(
    //     &mut self,
    //     #[base] _base: TRef<AnimationPlayer>,
    //     _name: String,
    // ) -> Option<()> {
    //     godot_dbg!("on_animation_finished");
    //     // if let Some(texture) = self.backwards_texture.take() {
    //     // if self.middle {
    //     // base.set_texture(texture);
    //     //     self.play(Directional::Backwards);
    //     // } else {
    //     // base.set_visible(false);
    //     // }
    //     Some(())
    // }

    // #[method]
    // fn on_child_entered_tree(&mut self, #[base] base: TRef<AnimationPlayer>, node: Ref<Node>) {
    //     base.set_visible(true);
    //     self.backwards_texture = img_texture_from_node(node);
    // }

    // #[method]
    // fn on_child_exiting_tree(&mut self, #[base] base: TRef<AnimationPlayer>, _node: Ref<Node>) {
    //     godot_dbg!("on_child_exiting_tree");
    //     // base.set_texture(img_texture_from_node(node).unwrap());
    //     // base.set_visible(true);
    //     // self.play();
    //     base.play("Pixelate", -1., 1.0, false);

    //     // self.play(Directional::Forwards);
    // }
}
// fn img_texture_from_node(node: Ref<Node>) -> Option<Ref<ImageTexture, Unique>> {
//     img_texture_from_viewport(unsafe { node.assume_safe() }.get_viewport().unwrap())
// }

// fn img_texture_from_viewport(node: Ref<Viewport>) -> Option<Ref<ImageTexture, Unique>> {
//     let img = unsafe {
//         node.assume_safe()
//             .get_viewport()?
//             .assume_safe()
//             .get_texture()?
//             .assume_safe()
//             .get_data()?
//     };
//     unsafe { img.assume_safe() }.flip_y();
//     let capture = ImageTexture::new();
//     capture.create_from_image(img, 7);
//     Some(capture)
// }
