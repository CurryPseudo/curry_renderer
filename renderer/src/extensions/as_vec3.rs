use crate::*;
pub trait AsVec3 {
    fn as_vec3(&self) -> Vec3;
}
impl AsVec3 for egui::Color32 {
    fn as_vec3(&self) -> Vec3 {
        Vec3::new(
            self.r() as f32 / 255.0,
            self.g() as f32 / 255.0,
            self.b() as f32 / 255.0,
        )
    }
}
