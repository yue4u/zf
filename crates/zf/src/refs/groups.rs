use gdnative::{api::Area, prelude::TRef};

pub const PLAYER: &'static str = "player";
pub const ENEMY: &'static str = "enemy";
pub const RADAR: &'static str = "radar";
pub const TARGET_POINT: &'static str = "point";
pub const TARGET_POINT_HAZARD: &'static str = "hazard";
pub const LEVEL_INDICATOR: &'static str = "level_indicator";

#[derive(Debug)]
#[allow(dead_code, non_camel_case_types)]
pub enum Layer {
    ENV,
    PLAYER,
    ENEMY,
    PLAYER_FIRE,
    ENEMY_FIRE,
}

impl Layer {
    pub fn bit(&self) -> i64 {
        match self {
            Layer::ENV => 0,
            Layer::PLAYER => 1,
            Layer::ENEMY => 2,
            Layer::PLAYER_FIRE => 3,
            Layer::ENEMY_FIRE => 4,
        }
    }

    pub fn prepare_collision_for(&self, area: TRef<Area>) {
        match self {
            Layer::PLAYER_FIRE => {
                area.set_collision_layer_bit(Layer::PLAYER_FIRE.bit(), true);
                area.set_collision_mask_bit(Layer::PLAYER.bit(), false);
            }
            Layer::ENEMY_FIRE => {
                area.set_collision_layer_bit(Layer::ENEMY_FIRE.bit(), true);
                area.set_collision_mask_bit(Layer::ENEMY.bit(), false);
            }
            _ => {}
        }
    }
}
