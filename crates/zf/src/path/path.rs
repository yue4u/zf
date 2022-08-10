#[rustfmt::skip]
#[allow(dead_code)]
pub mod scenes {
    pub const ENVIRONMENT: &str = "res://scenes/Environment.tscn";
    pub const BASE: &str = "res://scenes/Base.tscn";
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
    pub const WRAPPER: &str = "/root/Scene/UI/MarginContainer/Wrapper";
    pub const COMMAND_RESULT: &str = "/root/Scene/UI/MarginContainer/Wrapper/CommandResult";
    pub const V_BOX_CONTAINER: &str = "/root/Scene/UI/MarginContainer/Wrapper/VBoxContainer";
    pub const COMMAND_HISTORY: &str = "/root/Scene/UI/MarginContainer/Wrapper/VBoxContainer/CommandHistory";
    pub const PERF_LABEL: &str = "/root/Scene/UI/MarginContainer/Wrapper/PerfLabel";
    pub const BOTTOM: &str = "/root/Scene/UI/MarginContainer/Wrapper/Bottom";
    pub const COMMAND_PALETTE: &str = "/root/Scene/UI/MarginContainer/Wrapper/Bottom/CommandPalette";
    pub const LABEL: &str = "/root/Scene/UI/MarginContainer/Wrapper/Bottom/CommandPalette/Label";
    pub const LINE_EDIT: &str = "/root/Scene/UI/MarginContainer/Wrapper/Bottom/CommandPalette/LineEdit";
    pub const HELP: &str = "/root/Scene/UI/MarginContainer/Wrapper/Bottom/Help";
    pub const MANAGERS: &str = "/root/Scene/Managers";
    pub const VM: &str = "/root/Scene/Managers/VM";
    pub const SYS: &str = "/root/Scene/Managers/Sys";
    pub const ENVIRONMENT: &str = "/root/Scene/Environment";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod t_dummy {
    pub const AREA: &str = "/root/Scene/Area";
    pub const COLLISION_SHAPE: &str = "/root/Scene/Area/CollisionShape";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod base_level {

}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod homing_missile {
    pub const MESH_INSTANCE: &str = "/root/Scene/MeshInstance";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod start_menu {
    pub const CAMERA: &str = "/root/Scene/Camera";
    pub const CENTER_CONTAINER: &str = "/root/Scene/CenterContainer";
    pub const TITLE: &str = "/root/Scene/CenterContainer/Title";
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
    pub const COLOR_RECT: &str = "/root/Scene/UI/MarginContainer/Control/Radar/ColorRect";
    pub const PAWN: &str = "/root/Scene/UI/MarginContainer/Control/Radar/pawn";
}

