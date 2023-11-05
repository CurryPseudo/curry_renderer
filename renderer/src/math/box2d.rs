use crate::*;
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Box2d {
    pub min: Vec2,
    pub max: Vec2,
}

impl Box2d {
    pub fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }
    pub fn from_center_size(center: Vec2, size: Vec2) -> Self {
        Self {
            min: center - size / 2.0,
            max: center + size / 2.0,
        }
    }
    pub fn min_ubox2d_outside(&self) -> UBox2d {
        UBox2d::new(
            self.min.floor().as_uvec2(),
            self.max.ceil().as_uvec2() + UVec2::ONE,
        )
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct UBox2d {
    pub min: UVec2,
    pub max: UVec2,
}

impl UBox2d {
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
