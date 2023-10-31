use crate::*;
mod frame_buffer;
pub use frame_buffer::*;
mod render_target;

pub struct SyncCpuRenderer {
    msaa_enable: bool,
    ssaa_enable: bool,
    frame_buffer: CpuFrameBuffer,
    last_frame_time: std::time::Duration,
}

pub struct CpuRenderCommandList {
    msaa_enable: bool,
}
impl RenderCommandList for CpuRenderCommandList {
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
        let color_image = target.as_egui_color_image_mut();
        let size = color_image.size;
        for y in 0..size[1] {
            for x in 0..size[0] {
                let p = vec2(x as f32, y as f32) + Vec2::splat(0.5);
                if self.msaa_enable {
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
}

impl Default for SyncCpuRenderer {
    fn default() -> Self {
        Self {
            msaa_enable: Default::default(),
            ssaa_enable: Default::default(),
            frame_buffer: CpuFrameBuffer::new(UVec2::ONE, 1),
            last_frame_time: Default::default(),
        }
    }
}

impl Renderer for SyncCpuRenderer {
    fn create_frame_buffer(&self, size: UVec2) -> Box<dyn FrameBuffer> {
        if self.ssaa_enable {
            Box::new(CpuFrameBuffer::new(size, 2))
        } else {
            Box::new(CpuFrameBuffer::new(size, 1))
        }
    }

    fn msaa_enable(&mut self) -> &mut bool {
        &mut self.msaa_enable
    }

    fn ssaa_enable(&mut self) -> &mut bool {
        &mut self.ssaa_enable
    }

    fn frame_size(&self) -> UVec2 {
        self.frame_buffer.size()
    }

    fn resize_frame(&mut self, new_size: UVec2) {
        self.frame_buffer.resize(new_size);
    }

    fn last_frame_time(&self) -> std::time::Duration {
        self.last_frame_time
    }

    fn present(&self, ctx: &egui::Context) -> egui::TextureId {
        self.frame_buffer.as_egui_texture_id(ctx)
    }

    fn render_current_frame_if_ready(
        &mut self,
        f: Box<dyn Fn(&dyn RenderCommandList, &mut dyn FrameBuffer)>,
    ) {
        let expect_super_sampled_scale = if self.ssaa_enable { 2 } else { 1 };
        let frame_buffer = &mut self.frame_buffer;
        if frame_buffer.super_sampled_scale != expect_super_sampled_scale {
            *frame_buffer = CpuFrameBuffer::new(frame_buffer.size(), expect_super_sampled_scale)
        }
        let frame_begin = std::time::Instant::now();
        let render_command_list = CpuRenderCommandList {
            msaa_enable: self.msaa_enable,
        };
        f(&render_command_list, frame_buffer as &mut dyn FrameBuffer);
        self.last_frame_time = frame_begin.elapsed();
    }
}

pub struct AsyncCpuRenderer {
    msaa_enable: bool,
    ssaa_enable: bool,
    frame_buffer: CpuFrameBuffer,
    last_frame_time: std::time::Duration,
}

impl Default for AsyncCpuRenderer {
    fn default() -> Self {
        Self {
            msaa_enable: Default::default(),
            ssaa_enable: Default::default(),
            frame_buffer: CpuFrameBuffer::new(UVec2::ONE, 1),
            last_frame_time: Default::default(),
        }
    }
}

impl Renderer for AsyncCpuRenderer {
    fn create_frame_buffer(&self, size: UVec2) -> Box<dyn FrameBuffer> {
        if self.ssaa_enable {
            Box::new(CpuFrameBuffer::new(size, 2))
        } else {
            Box::new(CpuFrameBuffer::new(size, 1))
        }
    }

    fn msaa_enable(&mut self) -> &mut bool {
        &mut self.msaa_enable
    }

    fn ssaa_enable(&mut self) -> &mut bool {
        &mut self.ssaa_enable
    }

    fn frame_size(&self) -> UVec2 {
        self.frame_buffer.size()
    }

    fn resize_frame(&mut self, new_size: UVec2) {
        self.frame_buffer.resize(new_size);
    }

    fn last_frame_time(&self) -> std::time::Duration {
        self.last_frame_time
    }

    fn present(&self, ctx: &egui::Context) -> egui::TextureId {
        self.frame_buffer.as_egui_texture_id(ctx)
    }

    fn render_current_frame_if_ready(
        &mut self,
        f: Box<dyn Fn(&dyn RenderCommandList, &mut dyn FrameBuffer)>,
    ) {
        let expect_super_sampled_scale = if self.ssaa_enable { 2 } else { 1 };
        let frame_buffer = &mut self.frame_buffer;
        if frame_buffer.super_sampled_scale != expect_super_sampled_scale {
            *frame_buffer = CpuFrameBuffer::new(frame_buffer.size(), expect_super_sampled_scale)
        }
        let frame_begin = std::time::Instant::now();
        let render_command_list = CpuRenderCommandList {
            msaa_enable: self.msaa_enable,
        };
        f(&render_command_list, frame_buffer as &mut dyn FrameBuffer);
        self.last_frame_time = frame_begin.elapsed();
    }
}

pub type CpuRenderer = AsyncCpuRenderer;
