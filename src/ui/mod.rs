pub mod console;
pub mod hierarchy;
pub mod inspector;
pub mod menu_bar;
pub mod theme;
pub mod viewport;

pub use menu_bar::MenuBarActions;

use egui_wgpu::Renderer;
use egui_winit::State;
use winit::window::Window;

use crate::core::{AppState, TimeTracker};
use crate::gfx::OrbitCamera;

pub struct UiManager {
    pub ctx: egui::Context,
    pub state: State,
    pub renderer: Renderer,
}

impl UiManager {
    pub fn new(
        window: &Window,
        device: &wgpu::Device,
        surface_format: wgpu::TextureFormat,
    ) -> Self {
        let ctx = egui::Context::default();
        theme::apply_studio_theme(&ctx);

        let state = State::new(
            ctx.clone(),
            egui::ViewportId::ROOT,
            window,
            Some(window.scale_factor() as f32),
            None,
            None,
        );

        let renderer = Renderer::new(device, surface_format, None, 1, false);

        Self {
            ctx,
            state,
            renderer,
        }
    }

    pub fn prepare_frame(
        &mut self,
        window: &Window,
        app_state: &mut AppState,
        camera: &mut OrbitCamera,
        time: &TimeTracker,
        surface_size: [u32; 2],
    ) -> (egui::FullOutput, MenuBarActions) {
        let raw_input = self.state.take_egui_input(window);

        let mut bar_actions = MenuBarActions::default();

        let full_output = self.ctx.run(raw_input, |ctx| {
            bar_actions = menu_bar::render_menu_bar(
                ctx,
                app_state,
                camera,
                #[cfg(not(target_arch = "wasm32"))]
                window,
            );
            hierarchy::render_hierarchy(ctx, app_state, camera);
            inspector::render_inspector(ctx, app_state, surface_size);
            console::render_console(ctx, app_state, time);
            viewport::render_viewport_hud(ctx, app_state, camera, surface_size);
        });

        (full_output, bar_actions)
    }
}
