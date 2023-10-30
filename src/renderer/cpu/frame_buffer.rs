use super::*;
pub struct CpuFrameBuffer {
    id: Uuid,
    pub(crate) color_image: egui::ColorImage,
    pub(crate) super_sampled_scale: u32,
}

impl CpuFrameBuffer {
    pub fn new(size: UVec2, super_sampled_scale: u32) -> Self {
        Self {
            id: Uuid::new_v4(),
            color_image: egui::ColorImage::new(
                (size * super_sampled_scale).as_array(),
                egui::Color32::BLACK,
            ),
            super_sampled_scale,
        }
    }
}

impl FrameBuffer for CpuFrameBuffer {
    fn as_render_target_mut(&mut self) -> &mut dyn RenderTarget {
        self
    }

    fn size(&self) -> UVec2 {
        self.color_image.size.as_uvec2() / self.super_sampled_scale
    }

    fn resize(&mut self, new_size: UVec2) {
        self.color_image = egui::ColorImage::new(
            (new_size * self.super_sampled_scale).as_array(),
            egui::Color32::BLACK,
        );
    }

    fn as_egui_texture_id(&self, ctx: &egui::Context) -> egui::TextureId {
        let mut downsampled = egui::ColorImage::new(
            (self.color_image.size.as_uvec2() / self.super_sampled_scale).as_array(),
            egui::Color32::BLACK,
        );
        let size = downsampled.size.as_uvec2();
        for x in 0..size.x {
            for y in 0..size.y {
                let mut pixel_sum = Vec3::ZERO;
                for dx in 0..self.super_sampled_scale {
                    for dy in 0..self.super_sampled_scale {
                        let x = x * self.super_sampled_scale + dx;
                        let y = y * self.super_sampled_scale + dy;
                        let pixel = self.color_image.pixels
                            [(y * (self.color_image.size[0] as u32) + x) as usize];
                        pixel_sum += pixel.as_vec3();
                    }
                }
                downsampled.pixels[(y * size.x + x) as usize] = (pixel_sum
                    / ((self.super_sampled_scale * self.super_sampled_scale) as f32))
                    .as_egui_color32();
            }
        }
        ctx.load_texture(
            self.id.to_string(),
            egui::ImageData::Color(Arc::new(downsampled)),
            egui::TextureOptions {
                magnification: egui::TextureFilter::Nearest,
                minification: egui::TextureFilter::Nearest,
            },
        )
        .id()
    }
}
