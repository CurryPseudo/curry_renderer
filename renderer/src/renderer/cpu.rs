use crate::*;
use egui::Color32;

mod frame_buffer;
pub use frame_buffer::*;

mod render_target;
pub use render_target::*;

pub struct SyncCpuRenderer {
    msaa_enable: bool,
    ssaa_enable: bool,
    frame_buffer: CpuFrameBuffer,
    last_frame_time: Duration,
}

pub struct CpuRenderCommandList {
    msaa_enable: bool,
    ssaa_enable: bool,
}
impl RenderCommandList for CpuRenderCommandList {
    fn create_render_target(&self, size: UVec2) -> Box<dyn RenderTarget> {
        Box::new(CpuRenderTarget::new(
            size,
            self.msaa_enable,
            if self.ssaa_enable { 2 } else { 1 },
        ))
    }

    fn clear(&self, target: &mut dyn RenderTarget) {
        let cpu_rt = target
            .as_any_mut()
            .downcast_mut::<CpuRenderTarget>()
            .unwrap();
        cpu_rt.for_each_image_mut(|image, _| {
            image.pixels.fill(egui::Color32::BLACK);
        });
    }

    fn draw_triangle2d(
        &self,
        triangle: &Triangle2d,
        colors: &[egui::Color32; 3],
        target: &mut dyn RenderTarget,
    ) {
        let image_scale = target.image_scale();
        let triangle = triangle.map(|p| p * image_scale);
        let triangle = &triangle;
        let cpu_rt = target
            .as_any_mut()
            .downcast_mut::<CpuRenderTarget>()
            .unwrap();
        cpu_rt.for_each_image_mut(|image, pixel_offset| {
            let size = image.size.as_uvec2();
            let aabb = triangle.aabb();
            let uaabb = aabb.min_ubox2d_outside();
            if let Some(uaabb) = uaabb.intersect(&UBox2d::new(UVec2::ZERO, size)) {
                for y in uaabb.min.y..uaabb.max.y {
                    for x in uaabb.min.x..uaabb.max.x {
                        let p = vec2(x as f32, y as f32) + pixel_offset;
                        if triangle.contains(p) {
                            let barycentric_coord = triangle.barycentric_coord(p);
                            let mut color_sum = Vec3::ZERO;
                            color_sum += colors[0].as_vec3() * barycentric_coord.x;
                            color_sum += colors[1].as_vec3() * barycentric_coord.y;
                            color_sum += colors[2].as_vec3() * barycentric_coord.z;
                            image.pixels[(y * size.x + x) as usize] = color_sum.as_egui_color32();
                        }
                    }
                }
            }
        });
    }

    fn draw_triangle3d(
        &self,
        triangle: &Triangle3d,
        colors: &[Color32; 3],
        target: &mut dyn RenderTarget,
    ) {
        let image_scale = target.image_scale();
        let triangle = triangle.map(|p| p * image_scale);
        let triangle = &triangle;
        let cpu_rt = target
            .as_any_mut()
            .downcast_mut::<CpuRenderTarget>()
            .unwrap();
        cpu_rt.for_each_image_mut(|image, pixel_offset| {
            let size = image.size.as_uvec2();
            let aabb = triangle.project_to_xy().aabb();
            let uaabb = aabb.min_ubox2d_outside();
            if let Some(uaabb) = uaabb.intersect(&UBox2d::new(UVec2::ZERO, size)) {
                for y in uaabb.min.y..uaabb.max.y {
                    for x in uaabb.min.x..uaabb.max.x {
                        let p = vec2(x as f32, y as f32) + pixel_offset;
                        if triangle.project_to_xy().contains(p) {
                            let barycentric_coord = triangle.project_to_xy().barycentric_coord(p);
                            let mut color_sum = Vec3::ZERO;
                            color_sum += colors[0].as_vec3() * barycentric_coord.x;
                            color_sum += colors[1].as_vec3() * barycentric_coord.y;
                            color_sum += colors[2].as_vec3() * barycentric_coord.z;
                            image.pixels[(y * size.x + x) as usize] = color_sum.as_egui_color32();
                        }
                    }
                }
            }
        });
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
                        let source_x = x * source.super_sampled_scale + dx;
                        let source_y = y * source.super_sampled_scale + dy;
                        let mut current_pixel_sum = Vec3::ZERO;
                        source.for_each_image(|image, _| {
                            current_pixel_sum += image.pixels
                                [(source_y * (image.size[0] as u32) + source_x) as usize]
                                .as_vec3();
                        });
                        pixel_sum += current_pixel_sum / (source.image_count() as f32);
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

    fn last_frame_time(&self) -> Duration {
        self.last_frame_time
    }

    fn present(&self, ctx: &egui::Context) -> egui::TextureId {
        self.frame_buffer.as_egui_texture_id(ctx)
    }

    fn render_current_frame_if_ready(&mut self, f: RenderFrameFn) {
        let frame_buffer = &mut self.frame_buffer;
        let frame_begin = Instant::now();
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
    frame_time: Duration,
}
pub struct AsyncCpuRenderer {
    msaa_enable: bool,
    ssaa_enable: bool,
    frame_buffer: CpuFrameBuffer,
    last_frame_time: Duration,
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

    fn last_frame_time(&self) -> Duration {
        self.last_frame_time
    }

    fn present(&self, ctx: &egui::Context) -> egui::TextureId {
        self.frame_buffer.as_egui_texture_id(ctx)
    }

    fn render_current_frame_if_ready(&mut self, f: RenderFrameFn) {
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
            let frame_begin = Instant::now();
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

#[cfg(not(target_arch = "wasm32"))]
pub type CpuRenderer = AsyncCpuRenderer;

#[cfg(target_arch = "wasm32")]
pub type CpuRenderer = SyncCpuRenderer;
