use gdnative::{api::RichTextLabel, prelude::*};

use crate::common::{Position, Rotation, Vector3DisplayShort};

#[derive(NativeClass, Default)]
#[inherit(RichTextLabel)]
pub struct PlayerStatusDisplay {
    position: Position,
    rotation: Rotation,
    engine: EngineStatus,
}

#[derive(Debug)]
pub enum EngineStatus {
    On(u8),
    Off,
}

impl Default for EngineStatus {
    fn default() -> Self {
        EngineStatus::Off
    }
}

#[methods]
impl PlayerStatusDisplay {
    fn new(_owner: &RichTextLabel) -> Self {
        PlayerStatusDisplay::default()
    }

    #[export]
    fn _ready(&self, _owner: TRef<RichTextLabel>) -> Option<()> {
        godot_print!("player status ready");
        Some(())
    }

    #[export]
    fn _process(&mut self, owner: &RichTextLabel, _delta: f64) -> Option<()> {
        self.sync(owner)
    }

    fn sync(&mut self, owner: &RichTextLabel) -> Option<()> {
        let player = unsafe {
            owner
                .get_node("/root/Scene/Game/Path/PathFollow/t-mjolnir")?
                .assume_safe()
        }
        .cast::<Spatial>()?;

        self.position = player.translation();
        self.rotation = player.rotation_degrees();

        owner.set_bbcode(self.display());

        Some(())
    }

    fn display(&self) -> String {
        format!(
            r#"[b]Status[/b]
position: {}
rotation: {}

[b]Engine[/b]
engine: {:?}
"#,
            self.position.display(),
            self.rotation.display(),
            self.engine
        )
    }
}
