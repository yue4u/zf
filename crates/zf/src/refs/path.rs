#[rustfmt::skip]
#[allow(dead_code)]
pub mod scenes {
    pub const HEALTH_BAR_3_D: &str = "res://scenes/HealthBar3D.tscn";
    pub const ENVIRONMENT: &str = "res://scenes/Environment.tscn";
    pub const BASE: &str = "res://scenes/Base.tscn";
    pub const RADAR: &str = "res://scenes/Radar.tscn";
    pub const TERMINAL: &str = "res://scenes/Terminal.tscn";
    pub const HEALTH_BAR_2_D: &str = "res://scenes/HealthBar2D.tscn";
    pub const PLAYER_MJOLNIR: &str = "res://scenes/PlayerMjolnir.tscn";
    pub const BEAM: &str = "res://scenes/Beam.tscn";
    pub const T_DUMMY: &str = "res://scenes/T-Dummy.tscn";
    pub const UI_SANDBOX: &str = "res://scenes/UISandbox.tscn";
    pub const LAUNCHER: &str = "res://scenes/Launcher.tscn";
    pub const BASE_LEVEL: &str = "res://scenes/BaseLevel.tscn";
    pub const TARGET_DUMMY: &str = "res://scenes/TargetDummy.tscn";
    pub const TERMINAL_SANDBOX: &str = "res://scenes/TerminalSandbox.tscn";
    pub const TARGET_POINT: &str = "res://scenes/TargetPoint.tscn";
    pub const HOMING_MISSILE: &str = "res://scenes/HomingMissile.tscn";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod levels {
    pub const TUTORIAL_MOVEMENT: &str = "res://levels/Tutorial-Movement.tscn";
    pub const TUTORIAL_FIRE: &str = "res://levels/Tutorial-Fire.tscn";
    pub const START_MENU: &str = "res://levels/StartMenu.tscn";
    pub const SANDBOX: &str = "res://levels/Sandbox.tscn";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub enum SceneName {
    TutorialMovement,
    TutorialFire,
    StartMenu,
    Sandbox,
    Unknown,
}

impl From<&str> for SceneName {
    fn from(value: &str) -> Self {
        match value {
            levels::TUTORIAL_MOVEMENT => SceneName::TutorialMovement,
            levels::TUTORIAL_FIRE => SceneName::TutorialFire,
            levels::START_MENU => SceneName::StartMenu,
            levels::SANDBOX => SceneName::Sandbox,
            _ => SceneName::Unknown,
        }
    }
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
    pub const CONTROL: &str = "/root/Scene/UI/MarginContainer/Control";
    pub const PERF_LABEL: &str = "/root/Scene/UI/MarginContainer/Control/PerfLabel";
    pub const TERMINAL: &str = "/root/Scene/UI/MarginContainer/Control/Terminal";
    pub const UI_EXTRA: &str = "/root/Scene/UI/MarginContainer/UIExtra";
    pub const MANAGERS: &str = "/root/Scene/Managers";
    pub const VM: &str = "/root/Scene/Managers/VM";
    pub const ENVIRONMENT: &str = "/root/Scene/Environment";
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
pub mod terminal {

}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod health_bar_2_d {

}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod player_mjolnir {
    pub const CAMERA: &str = "/root/Scene/Camera";
    pub const RADAR_AREA: &str = "/root/Scene/RadarArea";
    pub const COLLISION_SHAPE: &str = "/root/Scene/RadarArea/CollisionShape";
    pub const PROJECTILES: &str = "/root/Scene/Projectiles";
    pub const AREA: &str = "/root/Scene/Area";
    pub const COLLISION_SHAPE_1: &str = "/root/Scene/Area/CollisionShape";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod beam {
    pub const AREA: &str = "/root/Scene/Area";
    pub const COLLISION_SHAPE: &str = "/root/Scene/Area/CollisionShape";
    pub const MESH_INSTANCE: &str = "/root/Scene/Area/MeshInstance";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod t_dummy {
    pub const SKETCHFAB_MODEL: &str = "/root/Scene/Sketchfab_model";
    pub const AREA: &str = "/root/Scene/Area";
    pub const COLLISION_SHAPE: &str = "/root/Scene/Area/CollisionShape";
    pub const HP: &str = "/root/Scene/HP";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod ui_sandbox {
    pub const ITEM_LIST: &str = "/root/Scene/UI/MarginContainer/Control/ItemList";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod launcher {

}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod base_level {
    pub const PROJECTILES: &str = "/root/Scene/Level/Projectiles";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod target_dummy {
    pub const SKETCHFAB_MODEL: &str = "/root/Scene/Sketchfab_model";
    pub const AREA: &str = "/root/Scene/Area";
    pub const COLLISION_SHAPE: &str = "/root/Scene/Area/CollisionShape";
    pub const HP: &str = "/root/Scene/HP";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod terminal_sandbox {
    pub const TERMINAL: &str = "/root/Scene/UI/MarginContainer/UIExtra/Terminal";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod target_point {
    pub const AREA: &str = "/root/Scene/Area";
    pub const COLLISION_SHAPE: &str = "/root/Scene/Area/CollisionShape";
    pub const CSG_MESH: &str = "/root/Scene/CSGMesh";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod homing_missile {
    pub const AREA: &str = "/root/Scene/Area";
    pub const MISSILES: &str = "/root/Scene/Area/missiles";
    pub const COLLISION_SHAPE: &str = "/root/Scene/Area/CollisionShape";
    pub const PARTICLES: &str = "/root/Scene/Area/Particles";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod tutorial_movement {
    pub const TARGET_POINT: &str = "/root/Scene/Level/TargetPoint";
    pub const PATH: &str = "/root/Scene/Level/Path";
    pub const PATH_FOLLOW: &str = "/root/Scene/Level/Path/PathFollow";
    pub const PLAYER_MJOLNIR: &str = "/root/Scene/Level/Path/PathFollow/PlayerMjolnir";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod tutorial_fire {
    pub const PLAYER_MJOLNIR: &str = "/root/Scene/Level/PlayerMjolnir";
    pub const T_DUMMY: &str = "/root/Scene/Level/t-dummy";
    pub const RADAR: &str = "/root/Scene/UI/MarginContainer/UIExtra/Radar";
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
    pub const AREA: &str = "/root/Scene/Level/Path/PathFollow/t-mjolnir/Area";
    pub const COLLISION_SHAPE_1: &str = "/root/Scene/Level/Path/PathFollow/t-mjolnir/Area/CollisionShape";
    pub const CSG_POLYGON: &str = "/root/Scene/Level/Path/CSGPolygon";
    pub const ENEMY_CLUSTER: &str = "/root/Scene/Level/EnemyCluster";
    pub const T_DUMMY_3: &str = "/root/Scene/Level/EnemyCluster/t-dummy3";
    pub const T_DUMMY_4: &str = "/root/Scene/Level/EnemyCluster/t-dummy4";
    pub const T_DUMMY_6: &str = "/root/Scene/Level/EnemyCluster/t-dummy6";
    pub const T_DUMMY_5: &str = "/root/Scene/Level/EnemyCluster/t-dummy5";
    pub const T_DUMMY_2: &str = "/root/Scene/Level/EnemyCluster/t-dummy2";
    pub const RADAR: &str = "/root/Scene/UI/MarginContainer/UIExtra/Radar";
    pub const UI_EXTRA: &str = "/root/Scene/UI/UIExtra";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod assets {
    pub const T_DUMMY_TSCN: &str = "res://assets/t-dummy.tscn";
    pub const JET_BRAINS_MONO_TRES: &str = "res://assets/JetBrains_Mono.tres";
    pub const BAR_PROGRESS_PNG: &str = "res://assets/bar_progress.png";
    pub const THEME_TRANSPARENT_TRES: &str = "res://assets/theme_transparent.tres";
    pub const BAR_UNDER_PNG: &str = "res://assets/bar_under.png";
    pub const T_DUMMY_TRES: &str = "res://assets/t-dummy.tres";
    pub const CODE_THEME_TRES: &str = "res://assets/code_theme.tres";
}

