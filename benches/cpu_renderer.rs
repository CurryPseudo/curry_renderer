use criterion::*;
use curry_renderer::SyncCpuRenderer;
use curry_renderer::*;
fn render_benchmark(c: &mut Criterion) {
    let mut cpu_renderer = SyncCpuRenderer::default();
    cpu_renderer.resize_frame(UVec2::new(1920, 1080));
    c.bench_function("render", |bencher| {
        bencher.iter(|| {
            let triangle_0 =
                Triangle::new(vec2(370.0, 320.0), vec2(490.0, 120.0), vec2(200.0, 220.0));
            let triangle_0_colors = [
                egui::Color32::RED,
                egui::Color32::BLUE,
                egui::Color32::YELLOW,
            ];
            let triangle_1 =
                Triangle::new(vec2(320.0, 370.0), vec2(120.0, 490.0), vec2(220.0, 200.0));
            let triangle_1_colors = [
                egui::Color32::BLUE,
                egui::Color32::GOLD,
                egui::Color32::GREEN,
            ];
            cpu_renderer.render_current_frame_if_ready(Box::new(move |cmd_list, fb| {
                let mut rt = cmd_list.create_render_target(fb.size());
                cmd_list.clear(rt.as_mut());
                cmd_list.draw_triangle(&triangle_0, &triangle_0_colors, rt.as_mut());
                cmd_list.draw_triangle(&triangle_1, &triangle_1_colors, rt.as_mut());
                cmd_list.copy_render_target_to_frame_buffer(rt.as_ref(), fb);
            }));
        });
    });
}
criterion_group!(benches, render_benchmark);
criterion_main!(benches);
