use crate::*;

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
