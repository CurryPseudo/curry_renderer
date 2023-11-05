use super::*;

pub enum CpuRenderTargetImage {
    Idle(egui::ColorImage),
    Multisampled([egui::ColorImage; 4]),
}
pub struct CpuRenderTarget {
    pub(crate) image: CpuRenderTargetImage,
    pub(crate) super_sampled_scale: u32,
}
impl CpuRenderTarget {
    pub fn new(size: UVec2, multisampled: bool, super_sampled_scale: u32) -> Self {
        let one_image = egui::ColorImage::new(
            (size * super_sampled_scale).as_array(),
            egui::Color32::BLACK,
        );
        Self {
            image: if multisampled {
                CpuRenderTargetImage::Multisampled([
                    one_image.clone(),
                    one_image.clone(),
                    one_image.clone(),
                    one_image,
                ])
            } else {
                CpuRenderTargetImage::Idle(one_image)
            },
            super_sampled_scale,
        }
    }
    pub fn image_count(&self) -> usize {
        match &self.image {
            CpuRenderTargetImage::Idle(_) => 1,
            CpuRenderTargetImage::Multisampled(_) => 4,
        }
    }
    pub fn for_each_image(&self, mut f: impl FnMut(&egui::ColorImage, Vec2)) {
        match &self.image {
            CpuRenderTargetImage::Idle(image) => f(image, Vec2::splat(0.5)),
            CpuRenderTargetImage::Multisampled(images) => {
                for dy in 0..2 {
                    for dx in 0..2 {
                        f(
                            &images[dy * 2 + dx],
                            vec2(dx as f32, dy as f32) * 0.5 + Vec2::splat(0.25),
                        );
                    }
                }
            }
        }
    }
    pub fn for_each_image_mut(&mut self, mut f: impl FnMut(&mut egui::ColorImage, Vec2)) {
        match &mut self.image {
            CpuRenderTargetImage::Idle(image) => f(image, Vec2::splat(0.5)),
            CpuRenderTargetImage::Multisampled(images) => {
                for dy in 0..2 {
                    for dx in 0..2 {
                        f(
                            &mut images[dy * 2 + dx],
                            vec2(dx as f32, dy as f32) * 0.5 + Vec2::splat(0.25),
                        );
                    }
                }
            }
        }
    }
}
impl Texture for CpuRenderTarget {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn size(&self) -> UVec2 {
        match &self.image {
            CpuRenderTargetImage::Idle(image) => image,
            CpuRenderTargetImage::Multisampled(images) => &images[0],
        }
        .size
        .as_uvec2()
            / self.super_sampled_scale
    }
}
impl RenderTarget for CpuRenderTarget {
    fn image_scale(&self) -> f32 {
        self.super_sampled_scale as f32
    }
}
