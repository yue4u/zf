use gdnative::prelude::{FromVariant, ToVariant};

#[rustfmt::skip]
#[allow(dead_code)]
pub mod scenes {
    pub const HEALTH_BAR_3_D: &str = "res://scenes/HealthBar3D.tscn";
    pub const ORBIT: &str = "res://scenes/Orbit.tscn";
    pub const ENVIRONMENT: &str = "res://scenes/Environment.tscn";
    pub const BASE: &str = "res://scenes/Base.tscn";
    pub const RADAR: &str = "res://scenes/Radar.tscn";
    pub const TERMINAL: &str = "res://scenes/Terminal.tscn";
    pub const TYPING_PARTICLES: &str = "res://scenes/TypingParticles.tscn";
    pub const PLANET_LAVA: &str = "res://scenes/PlanetLava.tscn";
    pub const HEALTH_BAR_2_D: &str = "res://scenes/HealthBar2D.tscn";
    pub const SANDBOX_2_D: &str = "res://scenes/Sandbox2D.tscn";
    pub const PLAYER_MJOLNIR: &str = "res://scenes/PlayerMjolnir.tscn";
    pub const SANDBOX_3_D: &str = "res://scenes/Sandbox3D.tscn";
    pub const TUTORIAL_COMPLETE: &str = "res://scenes/Tutorial_Complete.tscn";
    pub const PLAYER_STATUS: &str = "res://scenes/PlayerStatus.tscn";
    pub const PLAYER_HEALTH_BAR: &str = "res://scenes/PlayerHealthBar.tscn";
    pub const BEAM: &str = "res://scenes/Beam.tscn";
    pub const T_DUMMY: &str = "res://scenes/T-Dummy.tscn";
    pub const LAUNCHER: &str = "res://scenes/Launcher.tscn";
    pub const BASE_LEVEL: &str = "res://scenes/BaseLevel.tscn";
    pub const TITLE_LABEL: &str = "res://scenes/TitleLabel.tscn";
    pub const TARGET_DUMMY: &str = "res://scenes/TargetDummy.tscn";
    pub const MISSION: &str = "res://scenes/Mission.tscn";
    pub const TERMINAL_SANDBOX: &str = "res://scenes/TerminalSandbox.tscn";
    pub const SANDBOX: &str = "res://scenes/Sandbox.tscn";
    pub const TARGET_POINT: &str = "res://scenes/TargetPoint.tscn";
    pub const AUTO_LOAD: &str = "res://scenes/AutoLoad.tscn";
    pub const HOMING_MISSILE: &str = "res://scenes/HomingMissile.tscn";
    pub const PLANET: &str = "res://scenes/Planet.tscn";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod levels {
    pub const TUTORIAL_ENGINE_REL: &str = "res://levels/Tutorial_Engine_Rel.tscn";
    pub const CHALLENGE_INFINITE: &str = "res://levels/Challenge_Infinite.tscn";
    pub const TUTORIAL_TASK_MISSION_ENGINE_REL: &str = "res://levels/Tutorial_Task_Mission_Engine_Rel.tscn";
    pub const TUTORIAL_MISSION_ENGINE_REL: &str = "res://levels/Tutorial_Mission_Engine_Rel.tscn";
    pub const TUTORIAL_TASK_ENGINE_COMBINE: &str = "res://levels/Tutorial_Task_Engine_Combine.tscn";
    pub const CHALLENGE_ENGINE_REL: &str = "res://levels/Challenge_Engine_Rel.tscn";
    pub const TUTORIAL_FIRE: &str = "res://levels/Tutorial_Fire.tscn";
    pub const START_MENU: &str = "res://levels/StartMenu.tscn";
    pub const CHALLENGE_TASK_ENGINE_COMBINE: &str = "res://levels/Challenge_Task_Engine_Combine.tscn";
    pub const TUTORIAL_ENGINE: &str = "res://levels/Tutorial_Engine.tscn";
}

#[rustfmt::skip]
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, ToVariant, FromVariant)]
pub enum LevelName {
    TutorialEngineRel,
    ChallengeInfinite,
    TutorialTaskMissionEngineRel,
    TutorialMissionEngineRel,
    TutorialTaskEngineCombine,
    ChallengeEngineRel,
    TutorialFire,
    StartMenu,
    ChallengeTaskEngineCombine,
    TutorialEngine,
    Unknown,
}

impl LevelName {
    pub fn from_path(value: &str) -> Self {
        match value {
            levels::TUTORIAL_ENGINE_REL => LevelName::TutorialEngineRel,
            levels::CHALLENGE_INFINITE => LevelName::ChallengeInfinite,
            levels::TUTORIAL_TASK_MISSION_ENGINE_REL => LevelName::TutorialTaskMissionEngineRel,
            levels::TUTORIAL_MISSION_ENGINE_REL => LevelName::TutorialMissionEngineRel,
            levels::TUTORIAL_TASK_ENGINE_COMBINE => LevelName::TutorialTaskEngineCombine,
            levels::CHALLENGE_ENGINE_REL => LevelName::ChallengeEngineRel,
            levels::TUTORIAL_FIRE => LevelName::TutorialFire,
            levels::START_MENU => LevelName::StartMenu,
            levels::CHALLENGE_TASK_ENGINE_COMBINE => LevelName::ChallengeTaskEngineCombine,
            levels::TUTORIAL_ENGINE => LevelName::TutorialEngine,
            _ => LevelName::Unknown,
        }
    }
}

impl LevelName {
    pub fn path(&self) -> &'static str {
        match self {
            LevelName::TutorialEngineRel => levels::TUTORIAL_ENGINE_REL,
            LevelName::ChallengeInfinite => levels::CHALLENGE_INFINITE,
            LevelName::TutorialTaskMissionEngineRel => levels::TUTORIAL_TASK_MISSION_ENGINE_REL,
            LevelName::TutorialMissionEngineRel => levels::TUTORIAL_MISSION_ENGINE_REL,
            LevelName::TutorialTaskEngineCombine => levels::TUTORIAL_TASK_ENGINE_COMBINE,
            LevelName::ChallengeEngineRel => levels::CHALLENGE_ENGINE_REL,
            LevelName::TutorialFire => levels::TUTORIAL_FIRE,
            LevelName::StartMenu => levels::START_MENU,
            LevelName::ChallengeTaskEngineCombine => levels::CHALLENGE_TASK_ENGINE_COMBINE,
            LevelName::TutorialEngine => levels::TUTORIAL_ENGINE,
            LevelName::Unknown => unreachable!(),
        }
    }
}

impl LevelName {
    pub fn from(name: &str) -> LevelName {
        match name {
            "Tutorial-Engine-Rel" => LevelName::TutorialEngineRel,
            "Challenge-Infinite" => LevelName::ChallengeInfinite,
            "Tutorial-Task-Mission-Engine-Rel" => LevelName::TutorialTaskMissionEngineRel,
            "Tutorial-Mission-Engine-Rel" => LevelName::TutorialMissionEngineRel,
            "Tutorial-Task-Engine-Combine" => LevelName::TutorialTaskEngineCombine,
            "Challenge-Engine-Rel" => LevelName::ChallengeEngineRel,
            "Tutorial-Fire" => LevelName::TutorialFire,
            "Start-Menu" => LevelName::StartMenu,
            "Challenge-Task-Engine-Combine" => LevelName::ChallengeTaskEngineCombine,
            "Tutorial-Engine" => LevelName::TutorialEngine,
            _ => LevelName::Unknown,
        }
    }

    pub fn as_str(&self) -> &str {
        match &self {
            LevelName::TutorialEngineRel => "Tutorial-Engine-Rel",
            LevelName::ChallengeInfinite => "Challenge-Infinite",
            LevelName::TutorialTaskMissionEngineRel => "Tutorial-Task-Mission-Engine-Rel",
            LevelName::TutorialMissionEngineRel => "Tutorial-Mission-Engine-Rel",
            LevelName::TutorialTaskEngineCombine => "Tutorial-Task-Engine-Combine",
            LevelName::ChallengeEngineRel => "Challenge-Engine-Rel",
            LevelName::TutorialFire => "Tutorial-Fire",
            LevelName::StartMenu => "Start-Menu",
            LevelName::ChallengeTaskEngineCombine => "Challenge-Task-Engine-Combine",
            LevelName::TutorialEngine => "Tutorial-Engine",
            _ => "Unknown",
        }
    }
}

impl std::fmt::Display for LevelName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
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
pub mod orbit {

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
pub mod typing_particles {

}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod planet_lava {
    pub const SCENE_2: &str = "/root/Scene/scene2";
    pub const ANIMATION_PLAYER: &str = "/root/Scene/scene2/AnimationPlayer";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod health_bar_2_d {

}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod sandbox_2_d {
    pub const ITEM_LIST: &str = "/root/Scene/ItemList";
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
pub mod sandbox_3_d {
    pub const ORBIT: &str = "/root/Scene/Orbit";
    pub const IMMEDIATE_GEOMETRY: &str = "/root/Scene/Orbit/ImmediateGeometry";
    pub const MESH_11633: &str = "/root/Scene/Orbit/ImmediateGeometry/mesh11633";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod tutorial_complete {
    pub const ORBIT: &str = "/root/Scene/Level/Orbit";
    pub const PATH: &str = "/root/Scene/Level/Path";
    pub const PATH_FOLLOW: &str = "/root/Scene/Level/Path/PathFollow";
    pub const PLAYER_MJOLNIR: &str = "/root/Scene/Level/Path/PathFollow/PlayerMjolnir";
    pub const TARGET_POINT: &str = "/root/Scene/Level/Path/PathFollow/PlayerMjolnir/TargetPoint";
    pub const MARGIN_CONTAINER: &str = "/root/Scene/UI/MarginContainer";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod player_status {

}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod player_health_bar {
    pub const PLACEHOLDER: &str = "./placeholder";
    pub const PLACEHOLDER_2: &str = "./placeholder2";
    pub const CONTROL: &str = "./Control";
    pub const COLOR_RECT: &str = "./Control/ColorRect";
    pub const PLACEHOLDER_5: &str = "./placeholder5";
    pub const CURRENT: &str = "./Current";
    pub const LABEL_3: &str = "./Label3";
    pub const MAX: &str = "./Max";
    pub const PLACEHOLDER_6: &str = "./placeholder6";
    pub const CONTROL_2: &str = "./Control2";
    pub const COLOR_RECT_1: &str = "./Control2/ColorRect";
    pub const PLACEHOLDER_3: &str = "./placeholder3";
    pub const PLACEHOLDER_4: &str = "./placeholder4";
    pub const PROGRESS: &str = "./Progress";
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
pub mod launcher {

}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod base_level {
    pub const PROJECTILES: &str = "/root/Scene/Level/Projectiles";
    pub const LEVEL_RESULT: &str = "/root/Scene/UI/LevelResult";
    pub const UI_EXTRA: &str = "/root/Scene/UI/MarginContainer/UIExtra";
    pub const H_BOX_CONTAINER: &str = "/root/Scene/UI/MarginContainer/UIExtra/HBoxContainer";
    pub const MISSION: &str = "/root/Scene/UI/MarginContainer/UIExtra/HBoxContainer/Mission";
    pub const CONTROL_2: &str = "/root/Scene/UI/MarginContainer/UIExtra/HBoxContainer/Control2";
    pub const CONTROL_3: &str = "/root/Scene/UI/MarginContainer/UIExtra/HBoxContainer/Control3";
    pub const CONTROL: &str = "/root/Scene/UI/MarginContainer/UIExtra/HBoxContainer/Control";
    pub const PLAYER_STATUS: &str = "/root/Scene/UI/MarginContainer/UIExtra/HBoxContainer/PlayerStatus";
    pub const CONTROL_1: &str = "/root/Scene/UI/MarginContainer/UIExtra/Control";
    pub const H_BOX_CONTAINER_1: &str = "/root/Scene/UI/MarginContainer/UIExtra/Control/HBoxContainer";
    pub const LEVEL_NAME: &str = "/root/Scene/UI/MarginContainer/UIExtra/Control/LevelName";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod title_label {

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
pub mod mission {

}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod terminal_sandbox {
    pub const TERMINAL: &str = "/root/Scene/UI/MarginContainer/UIExtra/Terminal";
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
    pub const ORBIT: &str = "/root/Scene/Level/Path/Orbit";
    pub const ENEMY_CLUSTER: &str = "/root/Scene/Level/EnemyCluster";
    pub const T_DUMMY_3: &str = "/root/Scene/Level/EnemyCluster/t-dummy3";
    pub const T_DUMMY_4: &str = "/root/Scene/Level/EnemyCluster/t-dummy4";
    pub const T_DUMMY_6: &str = "/root/Scene/Level/EnemyCluster/t-dummy6";
    pub const T_DUMMY_5: &str = "/root/Scene/Level/EnemyCluster/t-dummy5";
    pub const T_DUMMY_2: &str = "/root/Scene/Level/EnemyCluster/t-dummy2";
    pub const UI_EXTRA: &str = "/root/Scene/UI/UIExtra";
    pub const RADAR: &str = "/root/Scene/UI/MarginContainer/UIExtra/Radar";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod target_point {
    pub const AREA: &str = "./Area";
    pub const COLLISION_SHAPE: &str = "./Area/CollisionShape";
    pub const CSG_MESH: &str = "./CSGMesh";
    pub const LABEL_3_D: &str = "./Label3D";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod auto_load {
    pub const UI: &str = "/root/AutoLoad/UI";
    pub const MARGIN_CONTAINER: &str = "/root/AutoLoad/UI/MarginContainer";
    pub const CONTROL: &str = "/root/AutoLoad/UI/MarginContainer/Control";
    pub const PERF_LABEL: &str = "/root/AutoLoad/UI/MarginContainer/Control/PerfLabel";
    pub const TERMINAL: &str = "/root/AutoLoad/UI/MarginContainer/Control/Terminal";
    pub const UI_EXTRA: &str = "/root/AutoLoad/UI/MarginContainer/UIExtra";
    pub const POST_PROCESSING_TEXTURE_RECT: &str = "/root/AutoLoad/UI/PostProcessingTextureRect";
    pub const SCREEN_TRANSITION_PLAYER: &str = "/root/AutoLoad/UI/PostProcessingTextureRect/ScreenTransitionPlayer";
    pub const MANAGERS: &str = "/root/AutoLoad/Managers";
    pub const VM: &str = "/root/AutoLoad/Managers/VM";
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
pub mod planet {
    pub const SCENE: &str = "/root/Scene/scene";
    pub const ANIMATION_PLAYER: &str = "/root/Scene/scene/AnimationPlayer";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod tutorial_engine_rel {
    pub const ORBIT: &str = "/root/Scene/Level/Orbit";
    pub const PATH: &str = "/root/Scene/Level/Path";
    pub const PATH_FOLLOW: &str = "/root/Scene/Level/Path/PathFollow";
    pub const POINT_1: &str = "/root/Scene/Level/Path/PathFollow/Point_1";
    pub const TARGET_1: &str = "/root/Scene/Level/Path/PathFollow/Point_1/Target_1";
    pub const PLAYER_MJOLNIR: &str = "/root/Scene/Level/Path/PathFollow/PlayerMjolnir";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod challenge_infinite {
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
    pub const ORBIT: &str = "/root/Scene/Level/Path/Orbit";
    pub const ENEMY_CLUSTER: &str = "/root/Scene/Level/EnemyCluster";
    pub const T_DUMMY_3: &str = "/root/Scene/Level/EnemyCluster/t-dummy3";
    pub const T_DUMMY_4: &str = "/root/Scene/Level/EnemyCluster/t-dummy4";
    pub const T_DUMMY_6: &str = "/root/Scene/Level/EnemyCluster/t-dummy6";
    pub const T_DUMMY_5: &str = "/root/Scene/Level/EnemyCluster/t-dummy5";
    pub const T_DUMMY_2: &str = "/root/Scene/Level/EnemyCluster/t-dummy2";
    pub const UI_EXTRA: &str = "/root/Scene/UI/UIExtra";
    pub const RADAR: &str = "/root/Scene/UI/MarginContainer/UIExtra/Radar";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod tutorial_task_mission_engine_rel {
    pub const ORBIT: &str = "/root/Scene/Level/Orbit";
    pub const PATH: &str = "/root/Scene/Level/Path";
    pub const PATH_FOLLOW: &str = "/root/Scene/Level/Path/PathFollow";
    pub const POINT_1: &str = "/root/Scene/Level/Path/PathFollow/Point_1";
    pub const TARGET_1: &str = "/root/Scene/Level/Path/PathFollow/Point_1/Target_1";
    pub const POINT_2: &str = "/root/Scene/Level/Path/PathFollow/Point_2";
    pub const TARGET_2: &str = "/root/Scene/Level/Path/PathFollow/Point_2/Target_2";
    pub const POINT_3: &str = "/root/Scene/Level/Path/PathFollow/Point_3";
    pub const TARGET_3: &str = "/root/Scene/Level/Path/PathFollow/Point_3/Target_3";
    pub const PLAYER_MJOLNIR: &str = "/root/Scene/Level/Path/PathFollow/PlayerMjolnir";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod tutorial_mission_engine_rel {
    pub const ORBIT: &str = "/root/Scene/Level/Orbit";
    pub const PATH: &str = "/root/Scene/Level/Path";
    pub const PATH_FOLLOW: &str = "/root/Scene/Level/Path/PathFollow";
    pub const POINT_1: &str = "/root/Scene/Level/Path/PathFollow/Point_1";
    pub const TARGET_1: &str = "/root/Scene/Level/Path/PathFollow/Point_1/Target_1";
    pub const POINT_2: &str = "/root/Scene/Level/Path/PathFollow/Point_2";
    pub const TARGET_2: &str = "/root/Scene/Level/Path/PathFollow/Point_2/Target_2";
    pub const POINT_3: &str = "/root/Scene/Level/Path/PathFollow/Point_3";
    pub const TARGET_3: &str = "/root/Scene/Level/Path/PathFollow/Point_3/Target_3";
    pub const PLAYER_MJOLNIR: &str = "/root/Scene/Level/Path/PathFollow/PlayerMjolnir";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod tutorial_task_engine_combine {
    pub const ORBIT: &str = "/root/Scene/Level/Orbit";
    pub const PATH: &str = "/root/Scene/Level/Path";
    pub const PATH_FOLLOW: &str = "/root/Scene/Level/Path/PathFollow";
    pub const PLAYER_MJOLNIR: &str = "/root/Scene/Level/Path/PathFollow/PlayerMjolnir";
    pub const POINT_1: &str = "/root/Scene/Level/Point_1";
    pub const TARGET_1: &str = "/root/Scene/Level/Point_1/Target_1";
    pub const POINT_2: &str = "/root/Scene/Level/Point_2";
    pub const TARGET_2: &str = "/root/Scene/Level/Point_2/Target_2";
    pub const POINT_3: &str = "/root/Scene/Level/Point_3";
    pub const TARGET_3: &str = "/root/Scene/Level/Point_3/Target_3";
    pub const POINT_4: &str = "/root/Scene/Level/Point_4";
    pub const TARGET_4: &str = "/root/Scene/Level/Point_4/Target_4";
    pub const POINT_5: &str = "/root/Scene/Level/Point_5";
    pub const TARGET_5: &str = "/root/Scene/Level/Point_5/Target_5";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod challenge_engine_rel {
    pub const ORBIT: &str = "/root/Scene/Level/Orbit";
    pub const PATH: &str = "/root/Scene/Level/Path";
    pub const PATH_FOLLOW: &str = "/root/Scene/Level/Path/PathFollow";
    pub const POINT_1: &str = "/root/Scene/Level/Path/PathFollow/Point_1";
    pub const TARGET_1: &str = "/root/Scene/Level/Path/PathFollow/Point_1/Target_1";
    pub const POINT_2: &str = "/root/Scene/Level/Path/PathFollow/Point_2";
    pub const TARGET_2: &str = "/root/Scene/Level/Path/PathFollow/Point_2/Target_2";
    pub const POINT_3: &str = "/root/Scene/Level/Path/PathFollow/Point_3";
    pub const TARGET_3: &str = "/root/Scene/Level/Path/PathFollow/Point_3/Target_3";
    pub const PLAYER_MJOLNIR: &str = "/root/Scene/Level/Path/PathFollow/PlayerMjolnir";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod tutorial_fire {
    pub const PLAYER_MJOLNIR: &str = "/root/Scene/Level/PlayerMjolnir";
    pub const T_DUMMY: &str = "/root/Scene/Level/t-dummy";
    pub const T_DUMMY_2: &str = "/root/Scene/Level/t-dummy2";
    pub const T_DUMMY_3: &str = "/root/Scene/Level/t-dummy3";
    pub const RADAR: &str = "/root/Scene/UI/MarginContainer/UIExtra/Radar";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod start_menu {
    pub const PLANET: &str = "/root/Scene/Level/Planet";
    pub const CENTER_CONTAINER: &str = "/root/Scene/Level/CenterContainer";
    pub const LEVEL_RESULT: &str = "/root/Scene/Level/CenterContainer/LevelResult";
    pub const OMNI_LIGHT: &str = "/root/Scene/Level/OmniLight";
    pub const CAMERA: &str = "/root/Scene/Level/Camera";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod challenge_task_engine_combine {
    pub const ORBIT: &str = "/root/Scene/Level/Orbit";
    pub const PLANET_2: &str = "/root/Scene/Level/Planet2";
    pub const PLANET: &str = "/root/Scene/Level/Planet";
    pub const PATH: &str = "/root/Scene/Level/Path";
    pub const PATH_FOLLOW: &str = "/root/Scene/Level/Path/PathFollow";
    pub const PLAYER_MJOLNIR: &str = "/root/Scene/Level/Path/PathFollow/PlayerMjolnir";
    pub const POINT_1: &str = "/root/Scene/Level/Point_1";
    pub const TARGET_1: &str = "/root/Scene/Level/Point_1/Target_1";
    pub const POINT_2: &str = "/root/Scene/Level/Point_2";
    pub const TARGET_2: &str = "/root/Scene/Level/Point_2/Target_2";
    pub const POINT_3: &str = "/root/Scene/Level/Point_3";
    pub const TARGET_3: &str = "/root/Scene/Level/Point_3/Target_3";
    pub const POINT_4: &str = "/root/Scene/Level/Point_4";
    pub const TARGET_4: &str = "/root/Scene/Level/Point_4/Target_4";
    pub const POINT_5: &str = "/root/Scene/Level/Point_5";
    pub const TARGET_5: &str = "/root/Scene/Level/Point_5/Target_5";
    pub const POINT_6: &str = "/root/Scene/Level/Point_6";
    pub const TARGET_6: &str = "/root/Scene/Level/Point_6/Target_6";
    pub const POINT_7: &str = "/root/Scene/Level/Point_7";
    pub const TARGET_7: &str = "/root/Scene/Level/Point_7/Target_7";
    pub const POINT_8: &str = "/root/Scene/Level/Point_8";
    pub const TARGET_8: &str = "/root/Scene/Level/Point_8/Target_8";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod tutorial_engine {
    pub const POINT_1: &str = "/root/Scene/Level/Point_1";
    pub const TARGET_1: &str = "/root/Scene/Level/Point_1/Target_1";
    pub const POINT_2: &str = "/root/Scene/Level/Point_2";
    pub const TARGET_2: &str = "/root/Scene/Level/Point_2/Target_2";
    pub const POINT_3: &str = "/root/Scene/Level/Point_3";
    pub const TARGET_3: &str = "/root/Scene/Level/Point_3/Target_3";
    pub const ORBIT: &str = "/root/Scene/Orbit";
    pub const PATH: &str = "/root/Scene/Path";
    pub const PATH_FOLLOW: &str = "/root/Scene/Path/PathFollow";
    pub const PLAYER_MJOLNIR: &str = "/root/Scene/Path/PathFollow/PlayerMjolnir";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod assets {
    pub const PATH_FOLLOW_SHADER: &str = "res://assets/PathFollow.shader";
    pub const UI_FONT_MENU_TITLE_TRES: &str = "res://assets/UIFontMenuTitle.tres";
    pub const T_DUMMY_TSCN: &str = "res://assets/t-dummy.tscn";
    pub const UI_THEME_TRES: &str = "res://assets/UITheme.tres";
    pub const JET_BRAINS_MONO_TRES: &str = "res://assets/JetBrains_Mono.tres";
    pub const PLAYER_HEALTH_BAR_SHADER: &str = "res://assets/PlayerHealthBar.shader";
    pub const BAR_PROGRESS_PNG: &str = "res://assets/bar_progress.png";
    pub const RADAR_TINT_SHADER: &str = "res://assets/radar_tint.shader";
    pub const RADAR_BG_SHADER: &str = "res://assets/radar_bg.shader";
    pub const THEME_TRANSPARENT_TRES: &str = "res://assets/theme_transparent.tres";
    pub const PLAYER_HEALTH_BAR_SHADER_TRES: &str = "res://assets/PlayerHealthBarShader.tres";
    pub const PIXELATE_TRES: &str = "res://assets/pixelate.tres";
    pub const BAR_UNDER_PNG: &str = "res://assets/bar_under.png";
    pub const TARGET_POINT_SHADER: &str = "res://assets/TargetPoint.shader";
    pub const UI_FONT_TITLE_TRES: &str = "res://assets/UIFontTitle.tres";
    pub const T_DUMMY_TRES: &str = "res://assets/t-dummy.tres";
    pub const UI_FONT_TRES: &str = "res://assets/UIFont.tres";
    pub const CODE_THEME_TRES: &str = "res://assets/code_theme.tres";
    pub const PIXELATE_SHADER: &str = "res://assets/pixelate.shader";
    pub const UI_TITLE_SHADER: &str = "res://assets/UITitle.shader";
}
