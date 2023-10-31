use crate::*;
mod frame_buffer;
pub use frame_buffer::*;

mod render_target;
pub use render_target::*;

pub struct SyncCpuRenderer {
    msaa_enable: bool,
    ssaa_enable: bool,
    frame_buffer: CpuFrameBuffer,
    last_frame_time: std::time::Duration,
}

pub struct CpuRenderCommandList {
    msaa_enable: bool,
    ssaa_enable: bool,
}
impl RenderCommandList for CpuRenderCommandList {
    fn create_render_target(&self, size: UVec2) -> Box<dyn RenderTarget> {
        Box::new(CpuRenderTarget::new(
            size,
            if self.ssaa_enable { 2 } else { 1 },
        ))
    }

    fn clear(&self, target: &mut dyn RenderTarget) {
        let color_image = target.as_egui_color_image_mut();
        color_image.pixels.fill(egui::Color32::BLACK);
    }

    fn draw_triangle(
        &self,
        triangle: &Triangle,
        color: egui::Color32,
        target: &mut dyn RenderTarget,
    ) {
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
                        color_image.pixels[y * size[0] + x] = egui::Color32::from_rgb(
                            (area_sum * color.r() as f32) as u8,
                            (area_sum * color.g() as f32) as u8,
                            (area_sum * color.g() as f32) as u8,
                        );
                    }
                } else {
                    if triangle.contains(p) {
                        color_image.pixels[y * size[0] + x] = color;
                    }
                }
            }
        }
    }

    fn copy_render_target_to_frame_buffer(
        &self,
        source: &dyn RenderTarget,
        destination: &mut dyn FrameBuffer,
    ) {
        assert_eq!(source.size(), destination.size());
        let source = source.as_any().downcast_ref::<CpuRenderTarget>().unwrap();
        let destination = destination
            .as_any_mut()
            .downcast_mut::<CpuFrameBuffer>()
            .unwrap();
        let downsampled = &mut destination.color_image;
        let size = downsampled.size.as_uvec2();
        for x in 0..size.x {
            for y in 0..size.y {
                let mut pixel_sum = Vec3::ZERO;
                for dx in 0..source.super_sampled_scale {
                    for dy in 0..source.super_sampled_scale {
                        let x = x * source.super_sampled_scale + dx;
                        let y = y * source.super_sampled_scale + dy;
                        let pixel = source.color_image.pixels
                            [(y * (source.color_image.size[0] as u32) + x) as usize];
                        pixel_sum += pixel.as_vec3();
                    }
                }
                downsampled.pixels[(y * size.x + x) as usize] = (pixel_sum
                    / ((source.super_sampled_scale * source.super_sampled_scale) as f32))
                    .as_egui_color32();
            }
        }
    }
}

impl Default for SyncCpuRenderer {
    fn default() -> Self {
        Self {
            msaa_enable: Default::default(),
            ssaa_enable: Default::default(),
            frame_buffer: CpuFrameBuffer::new(UVec2::ONE),
            last_frame_time: Default::default(),
        }
    }
}

impl Renderer for SyncCpuRenderer {
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
        self.frame_buffer = CpuFrameBuffer::new(new_size);
    }

    fn last_frame_time(&self) -> std::time::Duration {
        self.last_frame_time
    }

    fn present(&self, ctx: &egui::Context) -> egui::TextureId {
        self.frame_buffer.as_egui_texture_id(ctx)
    }

    fn render_current_frame_if_ready(
        &mut self,
        f: Box<dyn Fn(&dyn RenderCommandList, &mut dyn FrameBuffer) + Send>,
    ) {
        let frame_buffer = &mut self.frame_buffer;
        let frame_begin = std::time::Instant::now();
        let render_command_list = CpuRenderCommandList {
            msaa_enable: self.msaa_enable,
            ssaa_enable: self.ssaa_enable,
        };
        f(&render_command_list, frame_buffer as &mut dyn FrameBuffer);
        self.last_frame_time = frame_begin.elapsed();
    }
}

struct RenderThreadReturnData {
    frame_buffer: CpuFrameBuffer,
    frame_time: std::time::Duration,
}
pub struct AsyncCpuRenderer {
    msaa_enable: bool,
    ssaa_enable: bool,
    frame_buffer: CpuFrameBuffer,
    last_frame_time: std::time::Duration,
    render_thread: Option<std::thread::JoinHandle<RenderThreadReturnData>>,
}

impl Default for AsyncCpuRenderer {
    fn default() -> Self {
        Self {
            msaa_enable: Default::default(),
            ssaa_enable: Default::default(),
            frame_buffer: CpuFrameBuffer::new(UVec2::ONE),
            last_frame_time: Default::default(),
            render_thread: None,
        }
    }
}

impl Renderer for AsyncCpuRenderer {
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
        self.frame_buffer = CpuFrameBuffer::new(new_size);
    }

    fn last_frame_time(&self) -> std::time::Duration {
        self.last_frame_time
    }

    fn present(&self, ctx: &egui::Context) -> egui::TextureId {
        self.frame_buffer.as_egui_texture_id(ctx)
    }

    fn render_current_frame_if_ready(
        &mut self,
        f: Box<dyn Fn(&dyn RenderCommandList, &mut dyn FrameBuffer) + Send>,
    ) {
        let need_join = if let Some(handle) = &self.render_thread {
            handle.is_finished()
        } else {
            false
        };
        if need_join {
            let return_data = self.render_thread.take().unwrap().join().unwrap();
            if return_data.frame_buffer.size() == self.frame_buffer.size() {
                self.frame_buffer = return_data.frame_buffer;
                self.last_frame_time = return_data.frame_time;
            }
        }
        if self.render_thread.is_some() {
            return;
        }
        let size = self.frame_size();
        let msaa_enable = self.msaa_enable;
        let ssaa_enable = self.ssaa_enable;
        self.render_thread = Some(std::thread::spawn(move || {
            let mut frame_buffer = CpuFrameBuffer::new(size);
            let frame_begin = std::time::Instant::now();
            let render_command_list = CpuRenderCommandList {
                msaa_enable,
                ssaa_enable,
            };
            f(
                &render_command_list,
                &mut frame_buffer as &mut dyn FrameBuffer,
            );
            let frame_time = frame_begin.elapsed();
            RenderThreadReturnData {
                frame_buffer,
                frame_time,
            }
        }));
    }
}

pub type CpuRenderer = AsyncCpuRenderer;
