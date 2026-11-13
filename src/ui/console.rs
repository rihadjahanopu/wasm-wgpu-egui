use crate::core::{AppState, TimeTracker};
use egui::{Color32, Pos2, Stroke};

pub fn render_console(ctx: &egui::Context, state: &mut AppState, time: &TimeTracker) {
    if !state.ui_layout.show_console {
        return;
    }

    egui::TopBottomPanel::bottom("studio_console_panel")
        .default_height(140.0)
        .height_range(100.0..=240.0)
        .resizable(true)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("📊 Performance & Console");
                ui.separator();

                ui.label(
                    egui::RichText::new(format!("{:.1} FPS", time.fps()))
                        .color(if time.fps() > 50.0 {
                            Color32::GREEN
                        } else {
                            Color32::YELLOW
                        })
                        .strong(),
                );
                ui.label(format!("({:.2} ms)", time.frame_time_ms()));

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.small_button("Clear").clicked() {
                        state.logs.clear();
                    }
                });
            });

            ui.separator();

            ui.horizontal(|ui| {
                // Left side: Realtime Frame Time Sparkline
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new("Frame Time Graph (ms)").small().weak());
                    let (response, painter) =
                        ui.allocate_painter(egui::vec2(160.0, 50.0), egui::Sense::hover());
                    let rect = response.rect;

                    painter.rect_filled(rect, 4.0, Color32::from_rgb(18, 22, 28));
                    painter.rect_stroke(
                        rect,
                        4.0,
                        Stroke::new(1.0_f32, Color32::from_rgb(38, 44, 56)),
                    );

                    let history = time.frame_history();
                    if history.len() > 1 {
                        let max_ms: f32 = 33.3; // 30 FPS target line
                        let count = history.len() as f32;

                        let points: Vec<Pos2> = history
                            .iter()
                            .enumerate()
                            .map(|(i, &ms)| {
                                let x = rect.left() + (i as f32 / count) * rect.width();
                                let normalized_y = (ms / max_ms).clamp(0.0, 1.0);
                                let y = rect.bottom() - normalized_y * rect.height();
                                Pos2::new(x, y)
                            })
                            .collect();

                        for i in 0..points.len() - 1 {
                            painter.line_segment(
                                [points[i], points[i + 1]],
                                Stroke::new(1.5_f32, Color32::from_rgb(70, 170, 255)),
                            );
                        }

                        // 60 FPS (16.6ms) target line
                        let target_y = rect.bottom() - (16.6 / max_ms) * rect.height();
                        painter.line_segment(
                            [
                                Pos2::new(rect.left(), target_y),
                                Pos2::new(rect.right(), target_y),
                            ],
                            Stroke::new(
                                1.0_f32,
                                Color32::from_rgba_unmultiplied(100, 255, 100, 60),
                            ),
                        );
                    }
                });

                ui.separator();

                // Right side: Interactive Log Output Stream
                egui::ScrollArea::vertical()
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        for entry in &state.logs.entries {
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new(&entry.timestamp).weak().small());

                                let (badge, col) = match entry.level {
                                    log::Level::Error => ("ERR", Color32::from_rgb(255, 90, 90)),
                                    log::Level::Warn => ("WRN", Color32::from_rgb(255, 190, 60)),
                                    log::Level::Info => ("INF", Color32::from_rgb(70, 180, 255)),
                                    log::Level::Debug => ("DBG", Color32::GRAY),
                                    log::Level::Trace => ("TRC", Color32::DARK_GRAY),
                                };

                                ui.label(
                                    egui::RichText::new(format!("[{badge}]"))
                                        .color(col)
                                        .strong()
                                        .small(),
                                );
                                ui.label(egui::RichText::new(&entry.message).small());
                            });
                        }
                    });
            });
        });
}
