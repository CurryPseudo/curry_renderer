use crate::*;
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Rect {
    pub min: Vec2,
    pub max: Vec2,
}

impl Rect {
    pub fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }
    pub fn from_center_size(center: Vec2, size: Vec2) -> Self {
        Self {
            min: center - size / 2.0,
            max: center + size / 2.0,
        }
    }
    pub fn min_urect_outside(&self) -> URect {
        URect::new(
            self.min.floor().as_uvec2(),
            self.max.ceil().as_uvec2() + UVec2::ONE,
        )
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct URect {
    pub min: UVec2,
    pub max: UVec2,
}

impl URect {
    pub fn new(min: UVec2, max: UVec2) -> Self {
        Self { min, max }
    }
    pub fn from_center_size(center: UVec2, size: UVec2) -> Self {
        Self {
            min: center - size / 2,
            max: center + size / 2,
        }
    }
    pub fn intersect(&self, other: &Self) -> Option<Self> {
        let min = self.min.max(other.min);
        let max = self.max.min(other.max);
        if min.x < max.x && min.y < max.y {
            Some(Self { min, max })
        } else {
            None
        }
    }
}
