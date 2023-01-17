use gdnative::prelude::{FromVariant, ToVariant};

#[rustfmt::skip]
#[allow(dead_code)]
pub mod scenes {
    pub const ORBIT: &str = "res://scenes/Orbit.tscn";
    pub const ENVIRONMENT: &str = "res://scenes/Environment.tscn";
    pub const ENEMY_SPAWNER: &str = "res://scenes/EnemySpawner.tscn";
    pub const BASE: &str = "res://scenes/Base.tscn";
    pub const TERMINAL: &str = "res://scenes/Terminal.tscn";
    pub const MODEL_S_1: &str = "res://scenes/Model_S1.tscn";
    pub const TYPING_PARTICLES: &str = "res://scenes/TypingParticles.tscn";
    pub const PLANET_LAVA: &str = "res://scenes/PlanetLava.tscn";
    pub const SANDBOX_2_D: &str = "res://scenes/Sandbox2D.tscn";
    pub const PLAYER_MJOLNIR: &str = "res://scenes/PlayerMjolnir.tscn";
    pub const TUTORIAL_S_1: &str = "res://scenes/Tutorial_S1.tscn";
    pub const LEVEL_TIME: &str = "res://scenes/LevelTime.tscn";
    pub const SANDBOX_3_D: &str = "res://scenes/Sandbox3D.tscn";
    pub const ENEMY_HEALTH_BAR_3_D: &str = "res://scenes/EnemyHealthBar3D.tscn";
    pub const TUTORIAL_COMPLETE: &str = "res://scenes/Tutorial_Complete.tscn";
    pub const ENEMY_HEALTH_BAR_2_D: &str = "res://scenes/EnemyHealthBar2D.tscn";
    pub const PLAYER_STATUS: &str = "res://scenes/PlayerStatus.tscn";
    pub const PLAYER_HEALTH_BAR: &str = "res://scenes/PlayerHealthBar.tscn";
    pub const BEAM: &str = "res://scenes/Beam.tscn";
    pub const ENGINE_PARTICLES: &str = "res://scenes/EngineParticles.tscn";
    pub const LAUNCHER: &str = "res://scenes/Launcher.tscn";
    pub const BASE_LEVEL: &str = "res://scenes/BaseLevel.tscn";
    pub const TITLE_LABEL: &str = "res://scenes/TitleLabel.tscn";
    pub const COMMAND_INPUT_WATCHER: &str = "res://scenes/CommandInputWatcher.tscn";
    pub const TIME_TRIAL_TIMER: &str = "res://scenes/TimeTrialTimer.tscn";
    pub const ENEMY_S_1: &str = "res://scenes/Enemy_S1.tscn";
    pub const MISSION: &str = "res://scenes/Mission.tscn";
    pub const TERMINAL_SANDBOX: &str = "res://scenes/TerminalSandbox.tscn";
    pub const RADIATION_AREA: &str = "res://scenes/RadiationArea.tscn";
    pub const ITEM_LIST: &str = "res://scenes/ItemList.tscn";
    pub const SANDBOX: &str = "res://scenes/Sandbox.tscn";
    pub const TARGET_POINT: &str = "res://scenes/TargetPoint.tscn";
    pub const AUTO_LOAD: &str = "res://scenes/AutoLoad.tscn";
    pub const HOMING_MISSILE: &str = "res://scenes/HomingMissile.tscn";
    pub const SHIELD: &str = "res://scenes/Shield.tscn";
    pub const LEVEL_INDICATOR: &str = "res://scenes/LevelIndicator.tscn";
    pub const PLANET: &str = "res://scenes/Planet.tscn";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod levels {
    pub const TUTORIAL_RADAR_ENGINE_REL: &str = "res://levels/Tutorial_Radar_Engine_Rel.tscn";
    pub const CHALLENGE_SHIELD_RADIATION_AREA: &str = "res://levels/Challenge_Shield_RadiationArea.tscn";
    pub const TUTORIAL_TASK_RADAR_ENGINE_REL: &str = "res://levels/Tutorial_Task_Radar_Engine_Rel.tscn";
    pub const TUTORIAL_ENGINE_REL: &str = "res://levels/Tutorial_Engine_Rel.tscn";
    pub const TUTORIAL_ENEMY_APPEAR: &str = "res://levels/Tutorial_Enemy_Appear.tscn";
    pub const CHALLENGE_INFINITE: &str = "res://levels/Challenge_Infinite.tscn";
    pub const TUTORIAL_TASK_ENGINE_COMBINE: &str = "res://levels/Tutorial_Task_Engine_Combine.tscn";
    pub const CHALLENGE_SHIELD_RADIATION_AREA_TASK_ON: &str = "res://levels/Challenge_Shield_RadiationArea_Task_On.tscn";
    pub const CHALLENGE_ENGINE_REL: &str = "res://levels/Challenge_Engine_Rel.tscn";
    pub const TUTORIAL_SHIELD: &str = "res://levels/Tutorial_Shield.tscn";
    pub const TUTORIAL_FIRE: &str = "res://levels/Tutorial_Fire.tscn";
    pub const START_MENU: &str = "res://levels/StartMenu.tscn";
    pub const CHALLENGE_TASK_ENGINE_COMBINE: &str = "res://levels/Challenge_Task_Engine_Combine.tscn";
    pub const TUTORIAL_ENGINE: &str = "res://levels/Tutorial_Engine.tscn";
    pub const CHALLENGE_ENEMY_APPEAR: &str = "res://levels/Challenge_Enemy_Appear.tscn";
    pub const TUTORIAL_HELLO_WORLD: &str = "res://levels/Tutorial_Hello_World.tscn";
    pub const CHALLENGE_SHIELD: &str = "res://levels/Challenge_Shield.tscn";
}

#[rustfmt::skip]
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, ToVariant, FromVariant)]
pub enum LevelName {
    TutorialRadarEngineRel,
    ChallengeShieldRadiationArea,
    TutorialTaskRadarEngineRel,
    TutorialEngineRel,
    TutorialEnemyAppear,
    ChallengeInfinite,
    TutorialTaskEngineCombine,
    ChallengeShieldRadiationAreaTaskOn,
    ChallengeEngineRel,
    TutorialShield,
    TutorialFire,
    StartMenu,
    ChallengeTaskEngineCombine,
    TutorialEngine,
    ChallengeEnemyAppear,
    TutorialHelloWorld,
    ChallengeShield,
    Unknown,
}


impl LevelName {
    #[rustfmt::skip]
    pub fn from_path(value: &str) -> Self {
        match value {
            levels::TUTORIAL_RADAR_ENGINE_REL => LevelName::TutorialRadarEngineRel,
            levels::CHALLENGE_SHIELD_RADIATION_AREA => LevelName::ChallengeShieldRadiationArea,
            levels::TUTORIAL_TASK_RADAR_ENGINE_REL => LevelName::TutorialTaskRadarEngineRel,
            levels::TUTORIAL_ENGINE_REL => LevelName::TutorialEngineRel,
            levels::TUTORIAL_ENEMY_APPEAR => LevelName::TutorialEnemyAppear,
            levels::CHALLENGE_INFINITE => LevelName::ChallengeInfinite,
            levels::TUTORIAL_TASK_ENGINE_COMBINE => LevelName::TutorialTaskEngineCombine,
            levels::CHALLENGE_SHIELD_RADIATION_AREA_TASK_ON => LevelName::ChallengeShieldRadiationAreaTaskOn,
            levels::CHALLENGE_ENGINE_REL => LevelName::ChallengeEngineRel,
            levels::TUTORIAL_SHIELD => LevelName::TutorialShield,
            levels::TUTORIAL_FIRE => LevelName::TutorialFire,
            levels::START_MENU => LevelName::StartMenu,
            levels::CHALLENGE_TASK_ENGINE_COMBINE => LevelName::ChallengeTaskEngineCombine,
            levels::TUTORIAL_ENGINE => LevelName::TutorialEngine,
            levels::CHALLENGE_ENEMY_APPEAR => LevelName::ChallengeEnemyAppear,
            levels::TUTORIAL_HELLO_WORLD => LevelName::TutorialHelloWorld,
            levels::CHALLENGE_SHIELD => LevelName::ChallengeShield,
            _ => LevelName::Unknown,
        }
    }
}

impl LevelName {
    #[rustfmt::skip]
    pub fn path(&self) -> &'static str {
        match self {
            LevelName::TutorialRadarEngineRel => levels::TUTORIAL_RADAR_ENGINE_REL,
            LevelName::ChallengeShieldRadiationArea => levels::CHALLENGE_SHIELD_RADIATION_AREA,
            LevelName::TutorialTaskRadarEngineRel => levels::TUTORIAL_TASK_RADAR_ENGINE_REL,
            LevelName::TutorialEngineRel => levels::TUTORIAL_ENGINE_REL,
            LevelName::TutorialEnemyAppear => levels::TUTORIAL_ENEMY_APPEAR,
            LevelName::ChallengeInfinite => levels::CHALLENGE_INFINITE,
            LevelName::TutorialTaskEngineCombine => levels::TUTORIAL_TASK_ENGINE_COMBINE,
            LevelName::ChallengeShieldRadiationAreaTaskOn => levels::CHALLENGE_SHIELD_RADIATION_AREA_TASK_ON,
            LevelName::ChallengeEngineRel => levels::CHALLENGE_ENGINE_REL,
            LevelName::TutorialShield => levels::TUTORIAL_SHIELD,
            LevelName::TutorialFire => levels::TUTORIAL_FIRE,
            LevelName::StartMenu => levels::START_MENU,
            LevelName::ChallengeTaskEngineCombine => levels::CHALLENGE_TASK_ENGINE_COMBINE,
            LevelName::TutorialEngine => levels::TUTORIAL_ENGINE,
            LevelName::ChallengeEnemyAppear => levels::CHALLENGE_ENEMY_APPEAR,
            LevelName::TutorialHelloWorld => levels::TUTORIAL_HELLO_WORLD,
            LevelName::ChallengeShield => levels::CHALLENGE_SHIELD,
            LevelName::Unknown => unreachable!(),
        }
    }
}

impl LevelName {
    #[rustfmt::skip]
    pub fn from(name: &str) -> LevelName {
        match name {
            "Tutorial-Radar-Engine-Rel" => LevelName::TutorialRadarEngineRel,
            "Challenge-Shield-Radiation-Area" => LevelName::ChallengeShieldRadiationArea,
            "Tutorial-Task-Radar-Engine-Rel" => LevelName::TutorialTaskRadarEngineRel,
            "Tutorial-Engine-Rel" => LevelName::TutorialEngineRel,
            "Tutorial-Enemy-Appear" => LevelName::TutorialEnemyAppear,
            "Challenge-Infinite" => LevelName::ChallengeInfinite,
            "Tutorial-Task-Engine-Combine" => LevelName::TutorialTaskEngineCombine,
            "Challenge-Shield-Radiation-Area-Task-On" => LevelName::ChallengeShieldRadiationAreaTaskOn,
            "Challenge-Engine-Rel" => LevelName::ChallengeEngineRel,
            "Tutorial-Shield" => LevelName::TutorialShield,
            "Tutorial-Fire" => LevelName::TutorialFire,
            "Start-Menu" => LevelName::StartMenu,
            "Challenge-Task-Engine-Combine" => LevelName::ChallengeTaskEngineCombine,
            "Tutorial-Engine" => LevelName::TutorialEngine,
            "Challenge-Enemy-Appear" => LevelName::ChallengeEnemyAppear,
            "Tutorial-Hello-World" => LevelName::TutorialHelloWorld,
            "Challenge-Shield" => LevelName::ChallengeShield,
            _ => LevelName::Unknown,
        }
    }

    #[rustfmt::skip]
    pub fn as_str(&self) -> &str {
        match &self {
            LevelName::TutorialRadarEngineRel => "Tutorial-Radar-Engine-Rel",
            LevelName::ChallengeShieldRadiationArea => "Challenge-Shield-Radiation-Area",
            LevelName::TutorialTaskRadarEngineRel => "Tutorial-Task-Radar-Engine-Rel",
            LevelName::TutorialEngineRel => "Tutorial-Engine-Rel",
            LevelName::TutorialEnemyAppear => "Tutorial-Enemy-Appear",
            LevelName::ChallengeInfinite => "Challenge-Infinite",
            LevelName::TutorialTaskEngineCombine => "Tutorial-Task-Engine-Combine",
            LevelName::ChallengeShieldRadiationAreaTaskOn => "Challenge-Shield-Radiation-Area-Task-On",
            LevelName::ChallengeEngineRel => "Challenge-Engine-Rel",
            LevelName::TutorialShield => "Tutorial-Shield",
            LevelName::TutorialFire => "Tutorial-Fire",
            LevelName::StartMenu => "Start-Menu",
            LevelName::ChallengeTaskEngineCombine => "Challenge-Task-Engine-Combine",
            LevelName::TutorialEngine => "Tutorial-Engine",
            LevelName::ChallengeEnemyAppear => "Challenge-Enemy-Appear",
            LevelName::TutorialHelloWorld => "Tutorial-Hello-World",
            LevelName::ChallengeShield => "Challenge-Shield",
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
pub mod orbit {

}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod environment {
    pub const DIRECTIONAL_LIGHT: &str = "/root/Scene/DirectionalLight";
    pub const DIRECTIONAL_LIGHT_2: &str = "/root/Scene/DirectionalLight2";
    pub const WORLD_ENVIRONMENT: &str = "/root/Scene/WorldEnvironment";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod enemy_spawner {
    pub const AREA: &str = "/root/Scene/Area";
    pub const COLLISION_SHAPE: &str = "/root/Scene/Area/CollisionShape";
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
pub mod terminal {
    pub const SE_AUDIO_STREAM_PLAYER: &str = "/root/Scene/SEAudioStreamPlayer";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod model_s_1 {
    pub const CUBE_0: &str = "/root/Scene/Sketchfab_model/Root/Cube001/Cube_0";
    pub const ANIMATION_PLAYER: &str = "/root/Scene/AnimationPlayer";
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
pub mod sandbox_2_d {

}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod player_mjolnir {
    pub const ENGINE_PARTICLES: &str = "./Sketchfab_model/EngineParticles";
    pub const EL_1: &str = "./Sketchfab_model/EngineParticles/EL1";
    pub const EL_2: &str = "./Sketchfab_model/EngineParticles/EL2";
    pub const EL_3: &str = "./Sketchfab_model/EngineParticles/EL3";
    pub const EL_4: &str = "./Sketchfab_model/EngineParticles/EL4";
    pub const CAMERA: &str = "./Camera";
    pub const PROJECTILES: &str = "./Projectiles";
    pub const AREA: &str = "./Area";
    pub const COLLISION_SHAPE: &str = "./Area/CollisionShape";
    pub const SHIELD: &str = "./Shield";
    pub const ANIMATION_PLAYER: &str = "./AnimationPlayer";
    pub const SPOT_LIGHT: &str = "./SpotLight";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod tutorial_s_1 {
    pub const R_S_1: &str = "/root/Scene/R_S1";
    pub const AREA: &str = "/root/Scene/Area";
    pub const COLLISION_SHAPE: &str = "/root/Scene/Area/CollisionShape";
    pub const HP: &str = "/root/Scene/HP";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod level_time {

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
pub mod enemy_health_bar_3_d {
    pub const VIEWPORT: &str = "/root/Scene/Viewport";
    pub const CONTROL: &str = "/root/Scene/Viewport/Control";
    pub const SPRITE_3_D: &str = "/root/Scene/Sprite3D";
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
pub mod enemy_health_bar_2_d {

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
pub mod engine_particles {

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
    pub const M_PLACEHOLDER_1: &str = "/root/Scene/UI/MarginContainer/UIExtra/HBoxContainer/M_Placeholder_1";
    pub const M_PLACEHOLDER_2: &str = "/root/Scene/UI/MarginContainer/UIExtra/HBoxContainer/M_Placeholder_2";
    pub const M_PLACEHOLDER_3: &str = "/root/Scene/UI/MarginContainer/UIExtra/HBoxContainer/M_Placeholder_3";
    pub const PLAYER_STATUS: &str = "/root/Scene/UI/MarginContainer/UIExtra/HBoxContainer/PlayerStatus";
    pub const H_BOX_CONTAINER_2: &str = "/root/Scene/UI/MarginContainer/UIExtra/HBoxContainer2";
    pub const LISTENABLE_EVENT: &str = "/root/Scene/UI/MarginContainer/UIExtra/HBoxContainer2/ListenableEvent";
    pub const LE_PLACEHOLDER_1: &str = "/root/Scene/UI/MarginContainer/UIExtra/HBoxContainer2/LE_Placeholder_1";
    pub const LE_PLACEHOLDER_2: &str = "/root/Scene/UI/MarginContainer/UIExtra/HBoxContainer2/LE_Placeholder_2";
    pub const LE_PLACEHOLDER_3: &str = "/root/Scene/UI/MarginContainer/UIExtra/HBoxContainer2/LE_Placeholder_3";
    pub const LE_PLACEHOLDER_4: &str = "/root/Scene/UI/MarginContainer/UIExtra/HBoxContainer2/LE_Placeholder_4";
    pub const CONTROL: &str = "/root/Scene/UI/MarginContainer/UIExtra/Control";
    pub const H_BOX_CONTAINER_1: &str = "/root/Scene/UI/MarginContainer/UIExtra/Control/HBoxContainer";
    pub const LEVEL_NAME: &str = "/root/Scene/UI/MarginContainer/UIExtra/Control/LevelName";
    pub const LABEL: &str = "/root/Scene/UI/Label";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod title_label {

}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod command_input_watcher {

}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod time_trial_timer {

}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod enemy_s_1 {
    pub const R_S_1: &str = "/root/Scene/R_S1";
    pub const AREA: &str = "/root/Scene/Area";
    pub const COLLISION_SHAPE: &str = "/root/Scene/Area/CollisionShape";
    pub const HP: &str = "/root/Scene/HP";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod mission {
    pub const SE_AUDIO_STREAM_PLAYER: &str = "/root/Scene/SEAudioStreamPlayer";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod terminal_sandbox {
    pub const TERMINAL: &str = "/root/Scene/UI/MarginContainer/UIExtra/Terminal";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod radiation_area {
    pub const AREA: &str = "/root/Scene/Area";
    pub const COLLISION_SHAPE: &str = "/root/Scene/Area/CollisionShape";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod item_list {
    pub const ITEM_LIST: &str = "/root/Scene/ItemList";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod sandbox {
    pub const T_GANGUT_SPACE_HUB: &str = "/root/Scene/Level/t-gangut_space_hub";
    pub const PATH: &str = "/root/Scene/Level/Path";
    pub const PATH_FOLLOW: &str = "/root/Scene/Level/Path/PathFollow";
    pub const PLAYER_MJOLNIR: &str = "/root/Scene/Level/Path/PathFollow/PlayerMjolnir";
    pub const ORBIT: &str = "/root/Scene/Level/Path/Orbit";
    pub const ENEMY_CLUSTER: &str = "/root/Scene/Level/EnemyCluster";
    pub const T_DUMMY_3: &str = "/root/Scene/Level/EnemyCluster/t-dummy3";
    pub const T_DUMMY_4: &str = "/root/Scene/Level/EnemyCluster/t-dummy4";
    pub const T_DUMMY_6: &str = "/root/Scene/Level/EnemyCluster/t-dummy6";
    pub const T_DUMMY_5: &str = "/root/Scene/Level/EnemyCluster/t-dummy5";
    pub const T_DUMMY_2: &str = "/root/Scene/Level/EnemyCluster/t-dummy2";
    pub const UI_EXTRA: &str = "/root/Scene/UI/UIExtra";
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
    pub const SCREEN_TRANSITION_AUDIO_STREAM_PLAYER: &str = "/root/AutoLoad/UI/PostProcessingTextureRect/ScreenTransitionAudioStreamPlayer";
    pub const MANAGERS: &str = "/root/AutoLoad/Managers";
    pub const VM: &str = "/root/AutoLoad/Managers/VM";
    pub const BGM_PLAYER: &str = "/root/AutoLoad/BGMPlayer";
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
pub mod shield {
    pub const CSG_SPHERE: &str = "./CSGSphere";
    pub const ANIMATION_PLAYER: &str = "./CSGSphere/AnimationPlayer";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod level_indicator {

}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod planet {
    pub const SCENE: &str = "/root/Scene/scene";
    pub const ANIMATION_PLAYER: &str = "/root/Scene/scene/AnimationPlayer";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod tutorial_radar_engine_rel {
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
pub mod challenge_shield_radiation_area {
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
    pub const ORBIT: &str = "/root/Scene/Orbit";
    pub const SPATIAL: &str = "/root/Scene/Spatial";
    pub const SPATIAL_4: &str = "/root/Scene/Spatial4";
    pub const PLANET: &str = "/root/Scene/Planet";
    pub const PATH: &str = "/root/Scene/Path";
    pub const PATH_FOLLOW: &str = "/root/Scene/Path/PathFollow";
    pub const PLAYER_MJOLNIR: &str = "/root/Scene/Path/PathFollow/PlayerMjolnir";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod tutorial_task_radar_engine_rel {
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
pub mod tutorial_enemy_appear {
    pub const LEVEL_INDICATOR: &str = "/root/Scene/Level/LevelIndicator";
    pub const ORBIT: &str = "/root/Scene/Level/Orbit";
    pub const ENEMY_SPAWNER: &str = "/root/Scene/Level/EnemySpawner";
    pub const PATH: &str = "/root/Scene/Level/Path";
    pub const PATH_FOLLOW: &str = "/root/Scene/Level/Path/PathFollow";
    pub const PLAYER_MJOLNIR: &str = "/root/Scene/Level/Path/PathFollow/PlayerMjolnir";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod challenge_infinite {
    pub const T_GANGUT_SPACE_HUB: &str = "/root/Scene/Level/t-gangut_space_hub";
    pub const PATH: &str = "/root/Scene/Level/Path";
    pub const PATH_FOLLOW: &str = "/root/Scene/Level/Path/PathFollow";
    pub const PLAYER_MJOLNIR: &str = "/root/Scene/Level/Path/PathFollow/PlayerMjolnir";
    pub const ORBIT: &str = "/root/Scene/Level/Path/Orbit";
    pub const ENEMY_CLUSTER: &str = "/root/Scene/Level/EnemyCluster";
    pub const T_DUMMY_3: &str = "/root/Scene/Level/EnemyCluster/t-dummy3";
    pub const T_DUMMY_4: &str = "/root/Scene/Level/EnemyCluster/t-dummy4";
    pub const T_DUMMY_6: &str = "/root/Scene/Level/EnemyCluster/t-dummy6";
    pub const T_DUMMY_5: &str = "/root/Scene/Level/EnemyCluster/t-dummy5";
    pub const T_DUMMY_2: &str = "/root/Scene/Level/EnemyCluster/t-dummy2";
    pub const UI_EXTRA: &str = "/root/Scene/UI/UIExtra";
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
pub mod challenge_shield_radiation_area_task_on {
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
    pub const ORBIT: &str = "/root/Scene/Orbit";
    pub const SPATIAL: &str = "/root/Scene/Spatial";
    pub const SPATIAL_4: &str = "/root/Scene/Spatial4";
    pub const PATH: &str = "/root/Scene/Path";
    pub const PATH_FOLLOW: &str = "/root/Scene/Path/PathFollow";
    pub const PLAYER_MJOLNIR: &str = "/root/Scene/Path/PathFollow/PlayerMjolnir";
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
    pub const POINT_4: &str = "/root/Scene/Level/Path/PathFollow/Point_4";
    pub const TARGET_4: &str = "/root/Scene/Level/Path/PathFollow/Point_4/Target_4";
    pub const PLAYER_MJOLNIR: &str = "/root/Scene/Level/Path/PathFollow/PlayerMjolnir";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod tutorial_shield {
    pub const PLAYER_MJOLNIR: &str = "/root/Scene/Level/PlayerMjolnir";
    pub const T_DUMMY: &str = "/root/Scene/Level/t-dummy";
    pub const T_DUMMY_2: &str = "/root/Scene/Level/t-dummy2";
    pub const T_DUMMY_3: &str = "/root/Scene/Level/t-dummy3";
    pub const COMMAND_INPUT_WATCHER: &str = "/root/Scene/Level/CommandInputWatcher";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod tutorial_fire {
    pub const PLAYER_MJOLNIR: &str = "/root/Scene/Level/PlayerMjolnir";
    pub const T_DUMMY: &str = "/root/Scene/Level/PlayerMjolnir/t-dummy";
    pub const T_DUMMY_2: &str = "/root/Scene/Level/PlayerMjolnir/t-dummy2";
    pub const T_DUMMY_3: &str = "/root/Scene/Level/PlayerMjolnir/t-dummy3";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod start_menu {
    pub const PLANET: &str = "/root/Scene/Level/Planet";
    pub const CENTER_CONTAINER: &str = "/root/Scene/Level/CenterContainer";
    pub const TITLE: &str = "/root/Scene/Level/CenterContainer/Title";
    pub const TEXTURE_RECT: &str = "/root/Scene/Level/CenterContainer/Title/TextureRect";
    pub const OMNI_LIGHT: &str = "/root/Scene/Level/OmniLight";
    pub const CAMERA: &str = "/root/Scene/Level/Camera";
    pub const UI: &str = "/root/Scene/UI";
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
pub mod challenge_enemy_appear {
    pub const LEVEL_INDICATOR: &str = "/root/Scene/Level/LevelIndicator";
    pub const ORBIT: &str = "/root/Scene/Level/Orbit";
    pub const ENEMY_SPAWNER: &str = "/root/Scene/Level/EnemySpawner";
    pub const ENEMY_SPAWNER_2: &str = "/root/Scene/Level/EnemySpawner2";
    pub const ENEMY_SPAWNER_3: &str = "/root/Scene/Level/EnemySpawner3";
    pub const PATH: &str = "/root/Scene/Level/Path";
    pub const PATH_FOLLOW: &str = "/root/Scene/Level/Path/PathFollow";
    pub const PLAYER_MJOLNIR: &str = "/root/Scene/Level/Path/PathFollow/PlayerMjolnir";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod tutorial_hello_world {
    pub const PLAYER_MJOLNIR: &str = "/root/Scene/Level/PlayerMjolnir";
    pub const COMMAND_INPUT_WATCHER: &str = "/root/Scene/Level/CommandInputWatcher";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod challenge_shield {
    pub const PLAYER_MJOLNIR: &str = "/root/Scene/Level/PlayerMjolnir";
    pub const T_DUMMY: &str = "/root/Scene/Level/t-dummy";
    pub const T_DUMMY_2: &str = "/root/Scene/Level/t-dummy2";
    pub const T_DUMMY_3: &str = "/root/Scene/Level/t-dummy3";
    pub const T_DUMMY_4: &str = "/root/Scene/Level/t-dummy4";
    pub const T_DUMMY_5: &str = "/root/Scene/Level/t-dummy5";
    pub const TIME_TRIAL_TIMER: &str = "/root/Scene/Level/TimeTrialTimer";
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod assets {
    pub const ZF_TRANSPARENT_256_X_256_PNG: &str = "res://assets/ZF_Transparent_256x256.png";
    pub const PATH_FOLLOW_SHADER: &str = "res://assets/PathFollow.shader";
    pub const UI_FONT_MENU_TITLE_TRES: &str = "res://assets/UIFontMenuTitle.tres";
    pub const UI_THEME_TRES: &str = "res://assets/UITheme.tres";
    pub const PLAYER_HEALTH_BAR_SHADER: &str = "res://assets/PlayerHealthBar.shader";
    pub const ZF_PNG: &str = "res://assets/ZF.png";
    pub const RADIATION_AREA_SHADER: &str = "res://assets/RadiationArea.shader";
    pub const SHIELD_SHADER: &str = "res://assets/Shield.shader";
    pub const CODE_THEME_TRES: &str = "res://assets/CodeTheme.tres";
    pub const ZF_TITLE_BLEND_1: &str = "res://assets/ZF_Title.blend1";
    pub const ZF_BLEND_1: &str = "res://assets/ZF.blend1";
    pub const PLAYER_HEALTH_BAR_SHADER_TRES: &str = "res://assets/PlayerHealthBarShader.tres";
    pub const TERMINAL_FONT_TRES: &str = "res://assets/TerminalFont.tres";
    pub const PIXELATE_TRES: &str = "res://assets/Pixelate.tres";
    pub const ZF_TRANSPARENT_PNG: &str = "res://assets/ZF_Transparent.png";
    pub const ZF_BLEND: &str = "res://assets/ZF.blend";
    pub const ENEMY_BAR_PROGRESS_UNDER_PNG: &str = "res://assets/EnemyBarProgressUnder.png";
    pub const TARGET_POINT_SHADER: &str = "res://assets/TargetPoint.shader";
    pub const UI_FONT_TITLE_TRES: &str = "res://assets/UIFontTitle.tres";
    pub const UI_FONT_TRES: &str = "res://assets/UIFont.tres";
    pub const UI_TITLE_SHADER: &str = "res://assets/UITitle.shader";
    pub const ENEMY_BAR_PROGRESS_PNG: &str = "res://assets/EnemyBarProgress.png";
    pub const PIXELATE_SHADER: &str = "res://assets/Pixelate.shader";
    pub const DEFAULT_ENV_TRES: &str = "res://assets/DefaultEnv.tres";
    pub const PATH_FOLLOW_RANBOW_SHADER: &str = "res://assets/PathFollowRanbow.shader";
}

