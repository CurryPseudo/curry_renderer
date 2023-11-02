use crate::*;

#[derive(Clone, Copy)]
pub struct Triangle {
    pub a: Vec2,
    pub b: Vec2,
    pub c: Vec2,
}

impl Triangle {
    pub fn new(a: Vec2, b: Vec2, c: Vec2) -> Self {
        Self { a, b, c }
    }
    /// Not include point on the edge
    pub fn contains(&self, p: Vec2) -> bool {
        ((self.b - self.a).perp_dot(p - self.a) > 0.0
            && (self.c - self.b).perp_dot(p - self.b) > 0.0
            && (self.a - self.c).perp_dot(p - self.c) > 0.0)
            || ((self.b - self.a).perp_dot(p - self.a) < 0.0
                && (self.c - self.b).perp_dot(p - self.b) < 0.0
                && (self.a - self.c).perp_dot(p - self.c) < 0.0)
    }
    pub fn aabb(&self) -> Rect {
        let min = self.a.min(self.b).min(self.c);
        let max = self.a.max(self.b).max(self.c);
        Rect { min, max }
    }
    pub fn barycentric_coord(&self, p: Vec2) -> Vec3 {
        // https://gamedev.stackexchange.com/questions/23743/whats-the-most-efficient-way-to-find-barycentric-coordinates
        let ca = self.c - self.a;
        let ba = self.b - self.a;
        let pa = p - self.a;
        let d00 = ca.dot(ca);
        let d01 = ca.dot(ba);
        let d11 = ba.dot(ba);
        let d20 = pa.dot(ca);
        let d21 = pa.dot(ba);
        let denom = d00 * d11 - d01 * d01;
        let v = (d11 * d20 - d01 * d21) / denom;
        let w = (d00 * d21 - d01 * d20) / denom;
        let u = 1.0 - v - w;
        vec3(u, w, v)
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
