use crate::core::{AppState, MeshType};
use egui::CollapsingHeader;

pub fn render_inspector(ctx: &egui::Context, state: &mut AppState, surface_size: [u32; 2]) {
    if !state.ui_layout.show_inspector {
        return;
    }

    egui::SidePanel::right("node_inspector_panel")
        .default_width(280.0)
        .width_range(240.0..=360.0)
        .show(ctx, |ui| {
            ui.add_space(4.0);
            ui.heading("🛠 Inspector");
            ui.separator();

            CollapsingHeader::new("🔷 Active Geometry")
                .default_open(true)
                .show(ui, |ui| {
                    ui.label("Mesh Primitive:");
                    for m in MeshType::ALL {
                        if ui
                            .radio_value(&mut state.mesh_type, m, m.display_name())
                            .clicked()
                        {
                            state.logs.push(
                                log::Level::Info,
                                format!("Mesh changed to: {}", m.display_name()),
                            );
                        }
                    }
                });

            CollapsingHeader::new("📐 Transform")
                .default_open(true)
                .show(ui, |ui| {
                    ui.label("Scale:");
                    let mut uniform_scale = state.transform.scale.x;
                    if ui
                        .add(egui::Slider::new(&mut uniform_scale, 0.2..=3.0).text("Uniform"))
                        .changed()
                    {
                        state.transform.scale = glam::Vec3::splat(uniform_scale);
                    }

                    ui.label("Rotation Speed:");
                    ui.add(
                        egui::Slider::new(&mut state.transform.rotation_speed.y, 0.0..=4.0)
                            .text("Y Axis"),
                    );
                    ui.add(
                        egui::Slider::new(&mut state.transform.rotation_speed.x, 0.0..=4.0)
                            .text("X Axis"),
                    );

                    ui.checkbox(&mut state.transform.auto_rotate, "Continuous Auto-Rotation");
                });

            CollapsingHeader::new("🎨 Material & Lighting")
                .default_open(true)
                .show(ui, |ui| {
                    ui.label("Color Tint:");
                    let mut rgba = state.material.color_tint;
                    if ui.color_edit_button_rgba_unmultiplied(&mut rgba).changed() {
                        state.material.color_tint = rgba;
                    }

                    ui.add(
                        egui::Slider::new(&mut state.material.ambient_strength, 0.1..=1.0)
                            .text("Ambient Light"),
                    );
                });

            CollapsingHeader::new("🖥 Hardware & Surface")
                .default_open(false)
                .show(ui, |ui| {
                    ui.group(|ui| {
                        ui.label(format!("🎮 GPU: {}", state.adapter_name));
                        ui.label(format!("🔌 Backend: {}", state.adapter_backend));
                        ui.label(format!(
                            "📐 Viewport: {} × {} px",
                            surface_size[0], surface_size[1]
                        ));
                        ui.label("⚡ Architecture: WebAssembly + WebGPU");
                    });
                });

            ui.add_space(16.0);
            ui.separator();
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new("Developed by Rihad Jahan Opu")
                        .strong()
                        .small(),
                );
                ui.hyperlink_to("🌐 rihadjahanopu.com", "https://rihadjahanopu.com");
            });
        });
}
