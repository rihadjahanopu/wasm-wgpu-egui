use crate::core::AppState;
use crate::gfx::OrbitCamera;
use egui::CollapsingHeader;

pub fn render_hierarchy(ctx: &egui::Context, state: &mut AppState, camera: &mut OrbitCamera) {
    if !state.ui_layout.show_hierarchy {
        return;
    }

    egui::SidePanel::left("scene_hierarchy_panel")
        .default_width(220.0)
        .width_range(180.0..=300.0)
        .show(ctx, |ui| {
            ui.add_space(4.0);
            ui.heading("📂 Scene Tree");
            ui.separator();

            CollapsingHeader::new("🌐 World Environment")
                .default_open(true)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Clear:");
                        let mut rgb = [
                            state.environment.clear_color[0],
                            state.environment.clear_color[1],
                            state.environment.clear_color[2],
                        ];
                        if ui.color_edit_button_rgb(&mut rgb).changed() {
                            state.environment.clear_color[0] = rgb[0];
                            state.environment.clear_color[1] = rgb[1];
                            state.environment.clear_color[2] = rgb[2];
                        }
                    });
                });

            CollapsingHeader::new("🎥 Orbit Camera")
                .default_open(true)
                .show(ui, |ui| {
                    ui.label(format!("Distance: {:.2}m", camera.distance));
                    ui.label(format!(
                        "Yaw: {:.1}°, Pitch: {:.1}°",
                        camera.yaw.to_degrees(),
                        camera.pitch.to_degrees()
                    ));
                    if ui.small_button("Reset View").clicked() {
                        *camera = OrbitCamera::default();
                    }
                });

            CollapsingHeader::new(format!("🧊 Mesh: {}", state.mesh_type.display_name()))
                .default_open(true)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        if state.transform.auto_rotate {
                            if ui.button("⏸ Pause").clicked() {
                                state.transform.auto_rotate = false;
                            }
                        } else {
                            if ui.button("▶ Spin").clicked() {
                                state.transform.auto_rotate = true;
                            }
                        }
                        if ui.button("↺ Reset Pose").clicked() {
                            state.transform.rotation = glam::Vec3::ZERO;
                            state.transform.scale = glam::Vec3::ONE;
                        }
                    });
                });

            CollapsingHeader::new("💡 Directional Sun Light")
                .default_open(false)
                .show(ui, |ui| {
                    ui.label(format!(
                        "Dir: [{:.2}, {:.2}, {:.2}]",
                        state.environment.light_direction.x,
                        state.environment.light_direction.y,
                        state.environment.light_direction.z
                    ));
                    ui.add(
                        egui::Slider::new(&mut state.material.ambient_strength, 0.05..=1.0)
                            .text("Ambient"),
                    );
                });

            ui.add_space(12.0);
            ui.separator();
            ui.vertical_centered(|ui| {
                ui.label(egui::RichText::new("WGPU + Egui Engine").weak().small());
                ui.hyperlink_to(
                    egui::RichText::new("by Rihad Jahan Opu")
                        .color(egui::Color32::from_rgb(100, 180, 255))
                        .small(),
                    "https://rihadjahanopu.com",
                );
            });
        });
}
