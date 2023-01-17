use gdnative::{
    api::{object::ConnectFlags, ShaderMaterial, TextureRect},
    prelude::*,
};
use zf_ffi::CommandArgs;

use crate::{
    common::find_ref,
    entities::GameEvent,
    managers::VM,
    refs::path::player_health_bar,
    units::{Player, PLAYER_HIT},
    vm::{CommandInput, VMSignal},
};

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register_signal)]
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

    pub fn register_signal<T: NativeClass>(builder: &ClassBuilder<T>) {
        builder.signal(VMSignal::OnGameState.as_str()).done();
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

        let vm_manager = find_ref::<VM, Node>(base).unwrap();

        base.connect(
            VMSignal::OnGameState,
            vm_manager,
            VMSignal::OnGameState,
            VariantArray::new_shared(),
            ConnectFlags::DEFERRED.into(),
        )
        .expect("failed to connect vm");

        vm_manager
            .connect(
                VMSignal::OnCmdParsed,
                base,
                VMSignal::OnCmdParsed,
                VariantArray::new_shared(),
                ConnectFlags::DEFERRED.into(),
            )
            .expect("failed to connect vm");
    }

    #[method]
    fn on_cmd_parsed(&mut self, #[base] _base: &Node, input: CommandInput) {
        match input.cmd {
            CommandArgs::Repair => {
                self.current = self.current.saturating_add(200).min(self.max);
                self.prev = self.current as f64;

                unsafe { self.current_label.as_ref().unwrap().assume_safe() }
                    .set_text(self.current.to_string());
                self.update_shader_param();
            }
            _ => {}
        }
    }

    #[method]
    fn _process(&mut self, #[base] _base: &Node, delta: f64) {
        if self.prev > self.current as f64 {
            self.prev -= delta * 100.;
        }
        self.update_shader_param();
    }

    #[method]
    fn on_player_hit(&mut self, #[base] base: &Node, delta: u32) {
        self.update(base, delta);
    }

    pub fn update(&mut self, base: &Node, delta: u32) {
        self.prev = self.current as f64;
        self.current = self.current.saturating_sub(delta);

        unsafe { self.current_label.as_ref().unwrap().assume_safe() }
            .set_text(self.current.to_string());
        self.update_shader_param();
        if self.current == 0 {
            base.emit_signal(
                VMSignal::OnGameState,
                &[GameEvent::MissionFailed.to_variant()],
            );
        }
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
