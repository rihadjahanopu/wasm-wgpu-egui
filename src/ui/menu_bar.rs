use crate::core::{AppState, MeshType};
use crate::gfx::OrbitCamera;
use egui::menu;

/// Return value from render_menu_bar — signals the caller about window actions needed.
#[derive(Default)]
pub struct MenuBarActions {
    /// User clicked the custom close button — caller should exit event loop.
    pub request_close: bool,
    /// User is dragging in the title area — caller should call window.drag_window().
    pub request_drag: bool,
    /// User double-clicked the title area — caller should toggle maximize.
    pub request_toggle_maximize: bool,
}

pub fn render_menu_bar(
    ctx: &egui::Context,
    state: &mut AppState,
    camera: &mut OrbitCamera,
    #[cfg(not(target_arch = "wasm32"))] window: &winit::window::Window,
) -> MenuBarActions {
    #[allow(unused_mut)]
    let mut actions = MenuBarActions::default();

    if !state.ui_layout.show_menu_bar {
        return actions;
    }

    egui::TopBottomPanel::top("studio_menu_bar")
        .exact_height(28.0)
        .show(ctx, |ui| {
            menu::bar(ui, |ui| {
                // ── LEFT SIDE: Logo + Menus ──────────────────────────────
                ui.label(
                    egui::RichText::new("⚡ WGPU STUDIO")
                        .strong()
                        .color(egui::Color32::from_rgb(66, 150, 255)),
                );
                ui.separator();

                ui.menu_button("File", |ui| {
                    if ui.button("↺ Reset All Settings").clicked() {
                        let logs = std::mem::take(&mut state.logs);
                        *state = AppState::default();
                        state.logs = logs;
                        state
                            .logs
                            .push(log::Level::Info, "Settings restored to factory defaults");
                        *camera = OrbitCamera::default();
                        ui.close_menu();
                    }
                    if ui.button("🧹 Clear Logs").clicked() {
                        state.logs.clear();
                        ui.close_menu();
                    }
                });

                ui.menu_button("View", |ui| {
                    ui.checkbox(&mut state.ui_layout.show_hierarchy, "Scene Hierarchy");
                    ui.checkbox(&mut state.ui_layout.show_inspector, "Property Inspector");
                    ui.checkbox(&mut state.ui_layout.show_console, "Console & Metrics");
                    ui.checkbox(
                        &mut state.ui_layout.show_stats_badge,
                        "Viewport HUD Overlay",
                    );
                    ui.separator();
                    if ui.button("Center View on Mesh").clicked() {
                        camera.target = glam::Vec3::ZERO;
                        camera.distance = 3.2;
                        ui.close_menu();
                    }
                });

                ui.menu_button("Geometry", |ui| {
                    for m in MeshType::ALL {
                        let is_active = state.mesh_type == m;
                        if ui.selectable_label(is_active, m.display_name()).clicked() {
                            state.mesh_type = m;
                            state.logs.push(
                                log::Level::Info,
                                format!("Switched mesh to: {}", m.display_name()),
                            );
                            ui.close_menu();
                        }
                    }
                });

                ui.menu_button("Presets", |ui| {
                    if ui.button("🌌 Cyberpunk Neon").clicked() {
                        state.environment.clear_color = [0.03, 0.04, 0.08, 1.0];
                        state.material.color_tint = [1.0, 0.2, 0.8, 1.0];
                        state.transform.rotation_speed = glam::Vec3::new(0.6, 1.5, 0.3);
                        state
                            .logs
                            .push(log::Level::Info, "Applied Cyberpunk Neon preset");
                        ui.close_menu();
                    }
                    if ui.button("🌿 Mint Matrix").clicked() {
                        state.environment.clear_color = [0.02, 0.08, 0.04, 1.0];
                        state.material.color_tint = [0.2, 1.0, 0.6, 1.0];
                        state.transform.rotation_speed = glam::Vec3::new(0.3, 0.8, 0.2);
                        state
                            .logs
                            .push(log::Level::Info, "Applied Mint Matrix preset");
                        ui.close_menu();
                    }
                    if ui.button("🌅 Golden Hour").clicked() {
                        state.environment.clear_color = [0.12, 0.08, 0.06, 1.0];
                        state.material.color_tint = [1.0, 0.75, 0.3, 1.0];
                        state.transform.rotation_speed = glam::Vec3::new(0.4, 0.6, 0.1);
                        state
                            .logs
                            .push(log::Level::Info, "Applied Golden Hour preset");
                        ui.close_menu();
                    }
                });

                ui.menu_button("Help", |ui| {
                    ui.label(egui::RichText::new("⚡ WGPU Professional Engine Studio").strong());
                    ui.label("Navigation Controls:");
                    ui.label("• Left Click + Drag: Orbit Camera");
                    ui.label("• Mouse Scroll: Zoom In / Out");
                    ui.separator();
                    ui.label(
                        egui::RichText::new("👨‍💻 Developed by: Rihad Jahan Opu")
                            .strong()
                            .color(egui::Color32::from_rgb(100, 200, 255)),
                    );
                    ui.hyperlink_to("🌐 Visit rihadjahanopu.com", "https://rihadjahanopu.com");
                    ui.separator();
                    ui.hyperlink_to("📚 WGPU Specification", "https://wgpu.rs");
                });

                // ── CENTER: Draggable Title Region ───────────────────────
                let available = ui.available_width();
                let right_reserve = 160.0_f32;
                let drag_width = (available - right_reserve).max(0.0);

                let (drag_rect, drag_resp) = ui.allocate_exact_size(
                    egui::vec2(drag_width, ui.available_height()),
                    egui::Sense::click_and_drag(),
                );

                // Draw centered title label in the drag region
                ui.painter().text(
                    drag_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "⚡ WGPU Professional Engine Studio",
                    egui::FontId::proportional(11.0),
                    egui::Color32::from_rgb(140, 140, 160),
                );

                // On WASM drag_resp is unused — suppress warning
                let _ = &drag_resp;

                #[cfg(not(target_arch = "wasm32"))]
                {
                    if drag_resp.drag_started() {
                        actions.request_drag = true;
                    }
                    if drag_resp.double_clicked() {
                        actions.request_toggle_maximize = true;
                    }
                }

                // ── RIGHT SIDE: Author + Platform + Window Controls ──────
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // ── DESKTOP ONLY: Custom Window Buttons (CSD) ────────
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        // ✕ Close button — red on hover
                        let close_btn = ui.add(
                            egui::Button::new(egui::RichText::new("✕").size(12.0))
                                .min_size(egui::vec2(28.0, 22.0))
                                .frame(false),
                        );
                        if close_btn.hovered() {
                            ui.painter().rect_filled(
                                close_btn.rect,
                                egui::Rounding::same(3.0),
                                egui::Color32::from_rgb(196, 43, 28),
                            );
                            ui.painter().text(
                                close_btn.rect.center(),
                                egui::Align2::CENTER_CENTER,
                                "✕",
                                egui::FontId::proportional(12.0),
                                egui::Color32::WHITE,
                            );
                        }
                        if close_btn.clicked() {
                            actions.request_close = true;
                        }

                        ui.add_space(2.0);

                        // □ / 🗗 Maximize / Restore button
                        let is_maximized = window.is_maximized();
                        let max_icon = if is_maximized { "🗗" } else { "🗖" };
                        let max_btn = ui.add(
                            egui::Button::new(egui::RichText::new(max_icon).size(11.0))
                                .min_size(egui::vec2(28.0, 22.0))
                                .frame(false),
                        );
                        if max_btn.hovered() {
                            ui.painter().rect_filled(
                                max_btn.rect,
                                egui::Rounding::same(3.0),
                                egui::Color32::from_rgba_premultiplied(80, 80, 80, 180),
                            );
                        }
                        if max_btn.clicked() {
                            actions.request_toggle_maximize = true;
                        }

                        ui.add_space(2.0);

                        // — Minimize button
                        let min_btn = ui.add(
                            egui::Button::new(egui::RichText::new("—").size(12.0))
                                .min_size(egui::vec2(28.0, 22.0))
                                .frame(false),
                        );
                        if min_btn.hovered() {
                            ui.painter().rect_filled(
                                min_btn.rect,
                                egui::Rounding::same(3.0),
                                egui::Color32::from_rgba_premultiplied(80, 80, 80, 180),
                            );
                        }
                        if min_btn.clicked() {
                            window.set_minimized(true);
                        }

                        ui.separator();
                    }

                    // Version badge + author link (shown on both desktop and WASM)
                    #[cfg(not(target_arch = "wasm32"))]
                    ui.label(egui::RichText::new("v0.1.0 • Desktop").weak());
                    #[cfg(target_arch = "wasm32")]
                    ui.label(egui::RichText::new("v0.1.0 • WASM").weak());

                    ui.label(egui::RichText::new("•").weak());

                    ui.hyperlink_to(
                        egui::RichText::new("👨‍💻 Rihad Jahan Opu")
                            .color(egui::Color32::from_rgb(88, 166, 255))
                            .underline(),
                        "https://rihadjahanopu.com",
                    );
                });
            });
        });

    actions
}
