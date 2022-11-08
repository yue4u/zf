#[rustfmt::skip]
#[allow(dead_code)]
pub mod scenes {
    pub const HEALTH_BAR_3_D: &str = "res://scenes/HealthBar3D.tscn";
    pub const ENVIRONMENT: &str = "res://scenes/Environment.tscn";
    pub const BASE: &str = "res://scenes/Base.tscn";
    pub const RADAR: &str = "res://scenes/Radar.tscn";
    pub const HEALTH_BAR_2_D: &str = "res://scenes/HealthBar2D.tscn";
    pub const T_DUMMY: &str = "res://scenes/T-Dummy.tscn";
    pub const BASE_LEVEL: &str = "res://scenes/BaseLevel.tscn";
    pub const HOMING_MISSILE: &str = "res://scenes/HomingMissile.tscn";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod levels {
    pub const START_MENU: &str = "res://levels/StartMenu.tscn";
    pub const SANDBOX: &str = "res://levels/Sandbox.tscn";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod health_bar_3_d {
    pub const VIEWPORT: &str = "/root/Scene/Viewport";
    pub const CONTROL: &str = "/root/Scene/Viewport/Control";
    pub const SPRITE_3_D: &str = "/root/Scene/Sprite3D";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod environment {
    pub const DIRECTIONAL_LIGHT: &str = "/root/Scene/DirectionalLight";
    pub const WORLD_ENVIRONMENT: &str = "/root/Scene/WorldEnvironment";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod base {
    pub const LEVEL: &str = "/root/Scene/Level";
    pub const UI: &str = "/root/Scene/UI";
    pub const MARGIN_CONTAINER: &str = "/root/Scene/UI/MarginContainer";
    pub const UI_EXTRA: &str = "/root/Scene/UI/MarginContainer/UIExtra";
    pub const MANAGERS: &str = "/root/Scene/Managers";
    pub const VM: &str = "/root/Scene/Managers/VM";
    pub const ENVIRONMENT: &str = "/root/Scene/Environment";
    pub const LEGACY: &str = "/root/Scene/Legacy";
    pub const WRAPPER_LEGACY: &str = "/root/Scene/Legacy/WrapperLegacy";
    pub const COMMAND_RESULT: &str = "/root/Scene/Legacy/WrapperLegacy/CommandResult";
    pub const V_BOX_CONTAINER: &str = "/root/Scene/Legacy/WrapperLegacy/VBoxContainer";
    pub const COMMAND_HISTORY: &str = "/root/Scene/Legacy/WrapperLegacy/VBoxContainer/CommandHistory";
    pub const PERF_LABEL: &str = "/root/Scene/Legacy/WrapperLegacy/PerfLabel";
    pub const BOTTOM: &str = "/root/Scene/Legacy/WrapperLegacy/Bottom";
    pub const COMMAND_PALETTE: &str = "/root/Scene/Legacy/WrapperLegacy/Bottom/CommandPalette";
    pub const LABEL: &str = "/root/Scene/Legacy/WrapperLegacy/Bottom/CommandPalette/Label";
    pub const LINE_EDIT: &str = "/root/Scene/Legacy/WrapperLegacy/Bottom/CommandPalette/LineEdit";
    pub const TIPS: &str = "/root/Scene/Legacy/WrapperLegacy/Bottom/Tips";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod radar {
    pub const D_4: &str = "/root/Scene/D4";
    pub const BG: &str = "/root/Scene/BG";
    pub const TEXTURE_RECT: &str = "/root/Scene/TextureRect";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod health_bar_2_d {

}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod t_dummy {
    pub const SKETCHFAB_MODEL: &str = "/root/Scene/Sketchfab_model";
    pub const AREA: &str = "/root/Scene/Area";
    pub const COLLISION_SHAPE: &str = "/root/Scene/Area/CollisionShape";
    pub const HP: &str = "/root/Scene/HP";
    pub const LAUNCHER: &str = "/root/Scene/Launcher";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod base_level {
    pub const PROJECTILES: &str = "/root/Scene/Level/Projectiles";
    pub const CONTROL: &str = "/root/Scene/UI/MarginContainer/Control";
    pub const TERMINAL_WRAP: &str = "/root/Scene/UI/MarginContainer/Control/TerminalWrap";
    pub const TERMINAL: &str = "/root/Scene/UI/MarginContainer/Control/TerminalWrap/Terminal";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod homing_missile {
    pub const AREA: &str = "/root/Scene/Area";
    pub const MISSILES: &str = "/root/Scene/Area/missiles";
    pub const COLLISION_SHAPE: &str = "/root/Scene/Area/CollisionShape";
    pub const PARTICLES: &str = "/root/Scene/Area/Particles";
    pub const TIMER: &str = "/root/Scene/Timer";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod start_menu {
    pub const SCENE: &str = "/root/Scene/Level/scene";
    pub const ANIMATION_PLAYER: &str = "/root/Scene/Level/scene/AnimationPlayer";
    pub const CENTER_CONTAINER: &str = "/root/Scene/Level/CenterContainer";
    pub const TITLE: &str = "/root/Scene/Level/CenterContainer/Title";
    pub const OMNI_LIGHT: &str = "/root/Scene/Level/OmniLight";
    pub const CAMERA: &str = "/root/Scene/Level/Camera";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod sandbox {
    pub const T_GANGUT_SPACE_HUB: &str = "/root/Scene/Level/t-gangut_space_hub";
    pub const PATH: &str = "/root/Scene/Level/Path";
    pub const PATH_FOLLOW: &str = "/root/Scene/Level/Path/PathFollow";
    pub const T_MJOLNIR: &str = "/root/Scene/Level/Path/PathFollow/t-mjolnir";
    pub const CAMERA: &str = "/root/Scene/Level/Path/PathFollow/t-mjolnir/Camera";
    pub const RADAR_AREA: &str = "/root/Scene/Level/Path/PathFollow/t-mjolnir/RadarArea";
    pub const COLLISION_SHAPE: &str = "/root/Scene/Level/Path/PathFollow/t-mjolnir/RadarArea/CollisionShape";
    pub const PROJECTILES: &str = "/root/Scene/Level/Path/PathFollow/t-mjolnir/Projectiles";
    pub const CSG_POLYGON: &str = "/root/Scene/Level/Path/CSGPolygon";
    pub const ENEMY_CLUSTER: &str = "/root/Scene/Level/EnemyCluster";
    pub const T_DUMMY_3: &str = "/root/Scene/Level/EnemyCluster/t-dummy3";
    pub const T_DUMMY_4: &str = "/root/Scene/Level/EnemyCluster/t-dummy4";
    pub const T_DUMMY_6: &str = "/root/Scene/Level/EnemyCluster/t-dummy6";
    pub const T_DUMMY_5: &str = "/root/Scene/Level/EnemyCluster/t-dummy5";
    pub const T_DUMMY_2: &str = "/root/Scene/Level/EnemyCluster/t-dummy2";
    pub const TERMINAL: &str = "/root/Scene/UI/MarginContainer/Control/TerminalWrap/Terminal";
    pub const RADAR: &str = "/root/Scene/UI/MarginContainer/UIExtra/Radar";
    pub const UI_EXTRA: &str = "/root/Scene/UI/UIExtra";
}

