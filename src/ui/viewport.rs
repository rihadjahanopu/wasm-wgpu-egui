use crate::core::AppState;
use crate::gfx::OrbitCamera;
use egui::{Color32, Rounding, Stroke};

pub fn render_viewport_hud(
    ctx: &egui::Context,
    state: &AppState,
    camera: &OrbitCamera,
    surface_size: [u32; 2],
) {
    if !state.ui_layout.show_stats_badge {
        return;
    }

    // Top-right camera info overlay
    egui::Area::new(egui::Id::new("viewport_hud_overlay"))
        .anchor(egui::Align2::RIGHT_TOP, egui::vec2(-16.0, 40.0))
        .interactable(false)
        .show(ctx, |ui| {
            egui::Frame::none()
                .fill(Color32::from_rgba_unmultiplied(20, 24, 32, 200))
                .stroke(Stroke::new(
                    1.0_f32,
                    Color32::from_rgba_unmultiplied(60, 70, 90, 160),
                ))
                .rounding(Rounding::same(6.0))
                .inner_margin(8.0)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("🎥 3D Viewport")
                                .strong()
                                .color(Color32::from_rgb(100, 190, 255)),
                        );
                        ui.label(
                            egui::RichText::new("• 100% GPU Text")
                                .color(Color32::from_rgb(100, 255, 150))
                                .small(),
                        );
                    });
                    ui.label(format!("Camera Dist: {:.1}m", camera.distance));
                    ui.label(
                        egui::RichText::new("🖱 Left Drag: Orbit | Scroll: Zoom")
                            .small()
                            .weak(),
                    );
                });
        });

    // 3D In-World Projected Tag (Attached directly to 3D object in world space via GPU View-Projection matrix)
    let view_proj = camera.build_view_proj_matrix();
    let mesh_top_world = glam::Vec4::new(0.0, 0.95 * state.transform.scale.y, 0.0, 1.0);
    let clip = view_proj * mesh_top_world;

    if clip.w > 0.1 && surface_size[0] > 0 && surface_size[1] > 0 {
        let ndc_x = clip.x / clip.w;
        let ndc_y = clip.y / clip.w;

        // Convert NDC to screen points (taking pixels_per_point into account)
        let ppp = ctx.pixels_per_point();
        let screen_x = (ndc_x + 1.0) * 0.5 * (surface_size[0] as f32) / ppp;
        let screen_y = (1.0 - ndc_y) * 0.5 * (surface_size[1] as f32) / ppp - 24.0;

        egui::Area::new(egui::Id::new("3d_in_world_mesh_tag"))
            .fixed_pos(egui::pos2(screen_x - 75.0, screen_y))
            .interactable(false)
            .show(ctx, |ui| {
                egui::Frame::none()
                    .fill(Color32::from_rgba_unmultiplied(15, 20, 28, 230))
                    .stroke(Stroke::new(
                        1.0_f32,
                        Color32::from_rgba_unmultiplied(88, 166, 255, 200),
                    ))
                    .rounding(Rounding::same(12.0))
                    .inner_margin(egui::Margin::symmetric(10.0, 4.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("🧊").small());
                            ui.label(
                                egui::RichText::new(format!(
                                    "{} (GPU)",
                                    state.mesh_type.display_name()
                                ))
                                .color(Color32::WHITE)
                                .strong()
                                .small(),
                            );
                        });
                    });
            });
    }
}
