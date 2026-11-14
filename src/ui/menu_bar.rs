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
                    // ── DESKTOP ONLY: Vector-Drawn Window Controls (CSD) ────────
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        let button_size = egui::vec2(32.0, 22.0);
                        let icon_color = egui::Color32::from_rgb(205, 210, 220);
                        let hover_bg = egui::Color32::from_rgba_premultiplied(255, 255, 255, 22);

                        // 1. Close Button [✕]
                        let (close_rect, close_resp) =
                            ui.allocate_exact_size(button_size, egui::Sense::click());
                        let close_hovered = close_resp.hovered();
                        let close_stroke_color = if close_hovered {
                            egui::Color32::WHITE
                        } else {
                            icon_color
                        };
                        if close_hovered {
                            ui.painter().rect_filled(
                                close_rect,
                                egui::Rounding::same(3.0),
                                egui::Color32::from_rgb(196, 43, 28),
                            );
                        }
                        let c = close_rect.center();
                        let r = 4.0_f32;
                        let close_stroke = egui::Stroke::new(1.0_f32, close_stroke_color);
                        ui.painter().line_segment(
                            [c + egui::vec2(-r, -r), c + egui::vec2(r, r)],
                            close_stroke,
                        );
                        ui.painter().line_segment(
                            [c + egui::vec2(-r, r), c + egui::vec2(r, -r)],
                            close_stroke,
                        );
                        if close_resp.clicked() {
                            actions.request_close = true;
                        }

                        ui.add_space(2.0);

                        // 2. Maximize / Restore Button [□]
                        let is_maximized = window.is_maximized();
                        let (max_rect, max_resp) =
                            ui.allocate_exact_size(button_size, egui::Sense::click());
                        let max_hovered = max_resp.hovered();
                        let max_stroke_color = if max_hovered {
                            egui::Color32::WHITE
                        } else {
                            icon_color
                        };
                        if max_hovered {
                            ui.painter()
                                .rect_filled(max_rect, egui::Rounding::same(3.0), hover_bg);
                        }
                        let c = max_rect.center();
                        let max_stroke = egui::Stroke::new(1.0_f32, max_stroke_color);
                        if is_maximized {
                            // Two overlapping rectangles (Restore icon)
                            let back_rect = egui::Rect::from_min_size(
                                c + egui::vec2(-2.0, -4.5),
                                egui::vec2(7.0, 7.0),
                            );
                            ui.painter()
                                .rect_stroke(back_rect, egui::Rounding::ZERO, max_stroke);
                            let front_rect = egui::Rect::from_min_size(
                                c + egui::vec2(-4.5, -2.0),
                                egui::vec2(7.0, 7.0),
                            );
                            ui.painter().rect_filled(
                                front_rect,
                                egui::Rounding::ZERO,
                                egui::Color32::from_rgb(22, 27, 34),
                            );
                            ui.painter()
                                .rect_stroke(front_rect, egui::Rounding::ZERO, max_stroke);
                        } else {
                            // Single clean square outline (Maximize icon)
                            let sq = egui::Rect::from_center_size(c, egui::vec2(9.0, 9.0));
                            ui.painter()
                                .rect_stroke(sq, egui::Rounding::ZERO, max_stroke);
                        }
                        if max_resp.clicked() {
                            actions.request_toggle_maximize = true;
                        }

                        ui.add_space(2.0);

                        // 3. Minimize Button [—]
                        let (min_rect, min_resp) =
                            ui.allocate_exact_size(button_size, egui::Sense::click());
                        let min_hovered = min_resp.hovered();
                        let min_stroke_color = if min_hovered {
                            egui::Color32::WHITE
                        } else {
                            icon_color
                        };
                        if min_hovered {
                            ui.painter()
                                .rect_filled(min_rect, egui::Rounding::same(3.0), hover_bg);
                        }
                        let c = min_rect.center();
                        let min_stroke = egui::Stroke::new(1.0_f32, min_stroke_color);
                        ui.painter().line_segment(
                            [c + egui::vec2(-4.5, 0.0), c + egui::vec2(4.5, 0.0)],
                            min_stroke,
                        );
                        if min_resp.clicked() {
                            window.set_minimized(true);
                        }

                        ui.separator();
                    }

                    // Version badge + author link (shown on both desktop and WASM)
                    #[cfg(not(target_arch = "wasm32"))]
                    ui.label(
                        egui::RichText::new(concat!("v", env!("CARGO_PKG_VERSION"), " • Desktop"))
                            .weak(),
                    );
                    #[cfg(target_arch = "wasm32")]
                    ui.label(
                        egui::RichText::new(concat!("v", env!("CARGO_PKG_VERSION"), " • WASM"))
                            .weak(),
                    );

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
