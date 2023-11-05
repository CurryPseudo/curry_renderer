use crate::*;
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Box3d {
    pub min: Vec3,
    pub max: Vec3,
}

impl Box3d {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }
    pub fn from_center_size(center: Vec3, size: Vec3) -> Self {
        Self {
            min: center - size / 2.0,
            max: center + size / 2.0,
        }
    }
    pub fn min_ubox3d_outside(&self) -> UBox3d {
        UBox3d::new(
            self.min.floor().as_uvec3(),
            self.max.ceil().as_uvec3() + UVec3::ONE,
        )
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct UBox3d {
    pub min: UVec3,
    pub max: UVec3,
}

impl UBox3d {
    pub fn new(min: UVec3, max: UVec3) -> Self {
        Self { min, max }
    }
    pub fn from_center_size(center: UVec3, size: UVec3) -> Self {
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
