use std::collections::HashMap;

use crate::common::{Id, Position};

#[derive(Debug)]
pub struct Mission {
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

impl Mission {
    pub fn summary(self) -> String {
        format!("[b]{}[/b]\n\n{}", self.title, self.info)
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
