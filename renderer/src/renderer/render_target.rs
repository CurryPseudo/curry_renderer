use crate::*;
pub trait RenderTarget: Texture {
    fn image_scale(&self) -> f32;
}
