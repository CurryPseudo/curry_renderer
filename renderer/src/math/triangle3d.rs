use crate::*;

#[derive(Clone, Copy)]
pub struct Triangle3d([Vec3; 3]);

impl std::ops::Index<usize> for Triangle3d {
    type Output = Vec3;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for Triangle3d {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Triangle3d {
    pub fn new(a: Vec3, b: Vec3, c: Vec3) -> Self {
        Self([a, b, c])
    }
    pub fn aabb(&self) -> Box3d {
        let min = self[0].min(self[1]).min(self[2]);
        let max = self[0].max(self[1]).max(self[2]);
        Box3d { min, max }
    }
    pub fn map(self, f: impl Fn(Vec3) -> Vec3) -> Triangle3d {
        Self(self.0.map(f))
    }
    pub fn map_to_2d(self, f: impl Fn(Vec3) -> Vec2) -> Triangle2d {
        Triangle2d::new(f(self[0]), f(self[1]), f(self[2]))
    }
    pub fn project_to_xy(&self) -> Triangle2d {
        self.map_to_2d(|p| p.xy())
    }
}

#[test]
fn test_triangle_aabb() {
    let triangle = Triangle2d::new(vec2(-0.5, -0.5), vec2(1.0, 0.0), vec2(0.0, 1.0));
    assert_eq!(
        triangle.aabb(),
        Box2d::new(vec2(-0.5, -0.5), vec2(1.0, 1.0))
    );
}
