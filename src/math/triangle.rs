use crate::*;

#[derive(Clone, Copy)]
pub struct Triangle([Vec2; 3]);

impl std::ops::Index<usize> for Triangle {
    type Output = Vec2;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for Triangle {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Triangle {
    pub fn new(a: Vec2, b: Vec2, c: Vec2) -> Self {
        Self([a, b, c])
    }
    /// Not include point on the edge
    pub fn contains(&self, p: Vec2) -> bool {
        ((self[1] - self[0]).perp_dot(p - self[0]) > 0.0
            && (self[2] - self[1]).perp_dot(p - self[1]) > 0.0
            && (self[0] - self[2]).perp_dot(p - self[2]) > 0.0)
            || ((self[1] - self[0]).perp_dot(p - self[0]) < 0.0
                && (self[2] - self[1]).perp_dot(p - self[1]) < 0.0
                && (self[0] - self[2]).perp_dot(p - self[2]) < 0.0)
    }
    pub fn aabb(&self) -> Rect {
        let min = self[0].min(self[1]).min(self[2]);
        let max = self[0].max(self[1]).max(self[2]);
        Rect { min, max }
    }
    pub fn barycentric_coord(&self, p: Vec2) -> Vec3 {
        // https://gamedev.stackexchange.com/questions/23743/whats-the-most-efficient-way-to-find-barycentric-coordinates
        let v02 = self[2] - self[0];
        let v01 = self[1] - self[0];
        let v0p = p - self[0];
        let d00 = v02.dot(v02);
        let d01 = v02.dot(v01);
        let d11 = v01.dot(v01);
        let d20 = v0p.dot(v02);
        let d21 = v0p.dot(v01);
        let denom = d00 * d11 - d01 * d01;
        let v = (d11 * d20 - d01 * d21) / denom;
        let w = (d00 * d21 - d01 * d20) / denom;
        let u = 1.0 - v - w;
        vec3(u, w, v)
    }

    pub fn map(self, f: impl Fn(Vec2) -> Vec2) -> Triangle {
        Self(self.0.map(f))
    }
}

#[test]
fn test_triangle_contains() {
    let triangle = Triangle::new(vec2(0.0, 0.0), vec2(1.0, 0.0), vec2(0.0, 1.0));
    assert!(!triangle.contains(vec2(0.5, 0.5)));
    assert!(triangle.contains(vec2(0.4999, 0.4999)));
    assert!(triangle.contains(vec2(0.1, 0.3)));
    assert!(!triangle.contains(vec2(1.5, 0.5)));
    assert!(!triangle.contains(vec2(0.5, 1.5)));
    assert!(!triangle.contains(vec2(-0.5, 0.5)));
    assert!(!triangle.contains(vec2(0.5, -0.5)));
    assert!(triangle.contains(vec2(0.1, 0.899)));
}

#[test]
fn test_triangle_baycentric_coord() {
    let triangle = Triangle::new(vec2(0.0, 0.0), vec2(1.0, 0.0), vec2(0.0, 1.0));
    assert_eq!(
        triangle.barycentric_coord(vec2(0.0, 0.0)),
        vec3(1.0, 0.0, 0.0)
    );
    assert_eq!(
        triangle.barycentric_coord(vec2(1.0, 0.0)),
        vec3(0.0, 1.0, 0.0)
    );
    assert_eq!(
        triangle.barycentric_coord(vec2(0.0, 1.0)),
        vec3(0.0, 0.0, 1.0)
    );
    assert_eq!(
        triangle.barycentric_coord(vec2(0.5, 0.5)),
        vec3(0.0, 0.5, 0.5)
    );
    assert_eq!(
        triangle.barycentric_coord(vec2(0.0, 0.5)),
        vec3(0.5, 0.0, 0.5)
    );
}

#[test]
fn test_triangle_aabb() {
    let triangle = Triangle::new(vec2(-0.5, -0.5), vec2(1.0, 0.0), vec2(0.0, 1.0));
    assert_eq!(triangle.aabb(), Rect::new(vec2(-0.5, -0.5), vec2(1.0, 1.0)));
}
