use crate::*;
mod frame_buffer;
pub use frame_buffer::*;
mod render_target;

#[derive(Default)]
pub struct CpuRenderer {
    antialiasing_config: AntialiasingConfig,
}

impl Renderer for CpuRenderer {
    fn clear(&self, target: &mut dyn RenderTarget) {
        let color_image = target.as_egui_color_image_mut();
        color_image.pixels.fill(egui::Color32::BLACK);
    }

    fn draw_triangle(&self, triangle: &Triangle, target: &mut dyn RenderTarget) {
        let image_scale = target.image_scale();
        let triangle = Triangle {
            a: triangle.a * image_scale,
            b: triangle.b * image_scale,
            c: triangle.c * image_scale,
        };
        let triangle = &triangle;
        let AntialiasingConfig {
            msaa_enable,
            ssaa_enable: _,
        } = self.antialiasing_config;
        let color_image = target.as_egui_color_image_mut();
        let size = color_image.size;
        for y in 0..size[1] {
            for x in 0..size[0] {
                let p = vec2(x as f32, y as f32) + Vec2::splat(0.5);
                if msaa_enable {
                    let mut area_sum = 0.0;
                    for x in 0..2 {
                        for y in 0..2 {
                            let sub_p = p - Vec2::splat(0.25) + vec2(x as f32, y as f32) * 0.5;
                            if triangle.contains(sub_p) {
                                area_sum += 0.25;
                            }
                        }
                    }
                    if area_sum > 0.0 {
                        assert!(area_sum <= 1.0);
                        color_image.pixels[y * size[0] + x] =
                            egui::Color32::from_rgb((area_sum * 255.0) as u8, 0, 0);
                    }
                } else {
                    if triangle.contains(p) {
                        color_image.pixels[y * size[0] + x] = egui::Color32::RED;
                    }
                }
            }
        }
    }

    fn antialiasing_config_mut(&mut self) -> &mut AntialiasingConfig {
        &mut self.antialiasing_config
    }

    fn create_frame_buffer(&self, size: UVec2) -> Box<dyn FrameBuffer> {
        if self.antialiasing_config.ssaa_enable {
            Box::new(CpuFrameBuffer::new(size, 2))
        } else {
            Box::new(CpuFrameBuffer::new(size, 1))
        }
    }
}
