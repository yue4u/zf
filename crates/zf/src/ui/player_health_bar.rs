use gdnative::{
    api::{object::ConnectFlags, ShaderMaterial, TextureRect},
    prelude::*,
};

use crate::{
    refs::path::player_health_bar,
    units::{Player, PLAYER_HIT},
};

#[derive(NativeClass)]
#[inherit(Node)]
pub struct PlayerHealthBar {
    current: u32,
    prev: f64,
    max: u32,
    current_label: Option<Ref<Label>>,
    max_label: Option<Ref<Label>>,
    progress_rect: Option<Ref<TextureRect>>,
}

#[methods]
impl PlayerHealthBar {
    fn new(_base: &Node) -> Self {
        PlayerHealthBar {
            current: 5000,
            prev: 5000.,
            max: 5000,
            current_label: None,
            max_label: None,
            progress_rect: None,
        }
    }

    #[method]
    fn _ready(&mut self, #[base] base: TRef<Node>) {
        let current_label =
            unsafe { base.get_node_as::<Label>(player_health_bar::CURRENT) }.unwrap();
        let max_label = unsafe { base.get_node_as::<Label>(player_health_bar::MAX) }.unwrap();
        let progress_rect =
            unsafe { base.get_node_as::<TextureRect>(player_health_bar::PROGRESS) }.unwrap();

        current_label.set_text(self.current.to_string());
        max_label.set_text(self.max.to_string());

        self.current_label = Some(current_label.claim());
        self.max_label = Some(max_label.claim());
        self.progress_rect = Some(progress_rect.claim());
        self.update_shader_param();

        let player = unsafe {
            base.get_node_as_instance::<Player>(Player::path_from(base.as_ref()))
                .unwrap()
        };

        player
            .base()
            .connect(
                PLAYER_HIT,
                base,
                "on_player_hit",
                VariantArray::new_shared(),
                ConnectFlags::DEFERRED.into(),
            )
            .expect("failed to connect line edit");
    }

    #[method]
    fn _process(&mut self, #[base] _base: &Node, delta: f64) {
        if self.prev > self.current as f64 {
            self.prev -= delta * 100.;
        }
        self.update_shader_param();
    }

    #[method]
    fn on_player_hit(&mut self, delta: u32) {
        self.update(delta);
    }

    pub fn update(&mut self, delta: u32) {
        self.prev = self.current as f64;
        self.current = self.current.saturating_sub(delta);

        unsafe { self.current_label.as_ref().unwrap().assume_safe() }
            .set_text(self.current.to_string());
        self.update_shader_param();
    }

    fn update_shader_param(&self) {
        let val = self.current as f64 / self.max as f64;
        let prev_value = self.prev / self.max as f64;

        let texture_rect = unsafe {
            self.progress_rect
                .unwrap()
                .assume_safe()
                .material()
                .unwrap()
        };
        let shader = unsafe { texture_rect.assume_safe().cast::<ShaderMaterial>().unwrap() };
        shader.set_shader_param("value", val);
        shader.set_shader_param("prev_value", prev_value);
    }
}
