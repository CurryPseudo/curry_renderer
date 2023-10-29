use crate::*;
pub struct Rect {
    pub min: Vec2,
    pub max: Vec2,
}

impl Rect {
    pub fn from_center_size(center: Vec2, size: Vec2) -> Self {
        Self {
            min: center - size / 2.0,
            max: center + size / 2.0,
        }
    }
}
