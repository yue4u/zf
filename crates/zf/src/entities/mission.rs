use std::collections::HashMap;

use crate::{
    common::{Id, Position},
    refs::path::LevelName,
};
use gdnative::prelude::{FromVariant, ToVariant};
use nu_ansi_term::*;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct MissionLegacy {
    title: String,
    info: String,
    targets: TargetsMap,
}

pub type TargetsMap = HashMap<Id, MissionTarget>;

#[derive(Debug)]
pub struct MissionTarget {
    name: String,
    position: Position,
}

#[derive(Serialize, Deserialize)]
pub struct TargetPointInfo {
    pub name: String,
    pub pos: [f32; 3],
}

#[derive(Debug, ToVariant, FromVariant)]
pub enum GameEvent {
    HitTargetPoint,
    EnemyDestroied,
    MissionComplete(String),
    MissionFailed,
    LevelChange(LevelName),
}

impl MissionLegacy {
    pub fn summary(self) -> String {
        format!(
            "{}\n\n{}",
            Color::Cyan.bold().underline().paint(self.title),
            self.info
        )
    }

    pub fn targets(&self) -> Vec<String> {
        self.targets
            .values()
            .map(|t| format!("{} at {:?}", t.name, t.position))
            .collect()
    }

    pub fn positions(&self) -> Vec<Position> {
        self.targets.values().map(|t| t.position).collect()
    }

    pub fn dummy() -> Self {
        Self {
            title: "random mission".to_string(),
            info: "mission info".to_string(),
            targets: TargetsMap::from(([1, 2, 3]).map(|idx| {
                (
                    idx,
                    MissionTarget {
                        name: format!("target {idx}"),
                        position: Position::default(),
                    },
                )
            })),
        }
    }
}
