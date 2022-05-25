use gdnative::prelude::Vector3;

pub type Id = u32;
pub type Position = Vector3;
pub type Rotation = Vector3;

pub trait Vector3DisplayShort {
    fn display(&self) -> String;
}

impl Vector3DisplayShort for Vector3 {
    fn display(&self) -> String {
        format!("{:.2}, {:.2}, {:.2}", self.x, self.y, self.z)
    }
}
