use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy};
use winit::window::{Window, WindowId};

use crate::core::{AppState, TimeTracker};
use crate::gfx::{GpuContext, OrbitCamera, ScenePipeline, ShaderUniforms};
use crate::ui::UiManager;

pub enum AppEvent {
    GpuReady {
        gpu: GpuContext,
        pipeline: ScenePipeline,
        ui: UiManager,
    },
}

pub struct StudioApp {
    window: Option<Arc<Window>>,
    gpu: Option<GpuContext>,
    pipeline: Option<ScenePipeline>,
    ui: Option<UiManager>,
    camera: OrbitCamera,
    state: AppState,
    time: TimeTracker,
    proxy: EventLoopProxy<AppEvent>,

    // Mouse Navigation Tracking
    is_left_mouse_down: bool,
    last_cursor_pos: Option<[f32; 2]>,

    // CSD (Custom title bar) window action signals
    should_close: bool,
    should_toggle_maximize: bool,
    should_drag_window: bool,
}

impl StudioApp {
    pub fn new(proxy: EventLoopProxy<AppEvent>) -> Self {
        Self {
            window: None,
            gpu: None,
            pipeline: None,
            ui: None,
            camera: OrbitCamera::default(),
            state: AppState::default(),
            time: TimeTracker::new(),
            proxy,
            is_left_mouse_down: false,
            last_cursor_pos: None,
            should_close: false,
            should_toggle_maximize: false,
            should_drag_window: false,
        }
    }

    fn render_frame(&mut self, window: &Window) -> Result<(), wgpu::SurfaceError> {
        let (gpu, pipeline, ui) = match (&mut self.gpu, &mut self.pipeline, &mut self.ui) {
            (Some(g), Some(p), Some(u)) => (g, p, u),
            _ => return Ok(()),
        };

        // 1. Delta Time & Model Transform Updates
        let dt = self.time.tick();
        self.state.transform.update(dt);

        // 2. Check & Update Active Mesh (Zero runtime GPU allocation)
        pipeline.set_mesh(self.state.mesh_type);

        // 3. Update Camera & Aspect Ratio
        self.camera
            .update_aspect(gpu.config.width, gpu.config.height);
        let view_proj = self.camera.build_view_proj_matrix();
        let model = self.state.transform.build_matrix();

        let uniforms = ShaderUniforms {
            view_proj: view_proj.to_cols_array(),
            model: model.to_cols_array(),
            light_dir: self.state.environment.light_direction.to_array(),
            ambient_strength: self.state.material.ambient_strength,
            color_tint: self.state.material.color_tint,
        };
        pipeline.update_uniforms(&gpu.queue, &uniforms);

        // 4. Run Immediate-Mode GUI Frame
        let (full_output, bar_actions) = ui.prepare_frame(
            window,
            &mut self.state,
            &mut self.camera,
            &self.time,
            [gpu.config.width, gpu.config.height],
        );

        // Handle CSD signals from menu bar
        if bar_actions.request_close {
            self.should_close = true;
        }
        if bar_actions.request_toggle_maximize {
            self.should_toggle_maximize = true;
        }
        if bar_actions.request_drag {
            self.should_drag_window = true;
        }

        // Handle URL opening across platforms (especially on Web/WASM)
        if let Some(open_url) = &full_output.platform_output.open_url {
            #[cfg(target_arch = "wasm32")]
            {
                if let Some(win) = web_sys::window() {
                    let target = if open_url.new_tab { "_blank" } else { "_self" };
                    if let Err(e) = win.open_with_url_and_target(&open_url.url, target) {
                        log::error!("Failed to open link: {e:?}");
                    }
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                log::info!("Opening external URL: {}", open_url.url);
            }
        }

        ui.state
            .handle_platform_output(window, full_output.platform_output);

        let tris = ui
            .ctx
            .tessellate(full_output.shapes, full_output.pixels_per_point);

        for (id, delta) in &full_output.textures_delta.set {
            ui.renderer
                .update_texture(&gpu.device, &gpu.queue, *id, delta);
        }

        let screen_desc = egui_wgpu::ScreenDescriptor {
            size_in_pixels: [gpu.config.width, gpu.config.height],
            pixels_per_point: full_output.pixels_per_point,
        };

        // 5. Render Passes
        let output = gpu.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Studio Frame Master Command Encoder"),
            });

        ui.renderer
            .update_buffers(&gpu.device, &gpu.queue, &mut encoder, &tris, &screen_desc);

        // PASS 1: 3D Scene Geometry Pass with Depth Buffer
        {
            let mut scene_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Studio 3D Scene Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: self.state.environment.clear_color[0] as f64,
                            g: self.state.environment.clear_color[1] as f64,
                            b: self.state.environment.clear_color[2] as f64,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &gpu.depth_texture_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            let mesh = pipeline.active_mesh();
            scene_pass.set_pipeline(&pipeline.pipeline);
            scene_pass.set_bind_group(0, &pipeline.bind_group, &[]);
            scene_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
            scene_pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            scene_pass.draw_indexed(0..mesh.num_indices, 0, 0..1);
        }

        // PASS 2: Egui Immediate-Mode UI Pass (LoadOp::Load overlays cleanly on top)
        {
            let mut egui_pass = encoder
                .begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Studio Egui Overlay Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    occlusion_query_set: None,
                    timestamp_writes: None,
                })
                .forget_lifetime();

            ui.renderer.render(&mut egui_pass, &tris, &screen_desc);
        }

        gpu.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        for id in &full_output.textures_delta.free {
            ui.renderer.free_texture(id);
        }

        Ok(())
    }
}

impl ApplicationHandler<AppEvent> for StudioApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        // On native desktop: remove OS title bar → our egui menu bar acts as CSD
        #[cfg(not(target_arch = "wasm32"))]
        let attributes = Window::default_attributes()
            .with_title("⚡ WGPU Professional Engine Studio")
            .with_decorations(false);

        #[cfg(target_arch = "wasm32")]
        let attributes =
            Window::default_attributes().with_title("⚡ WGPU Professional Engine Studio");

        #[cfg(target_arch = "wasm32")]
        let attributes = {
            use web_sys::wasm_bindgen::JsCast;
            use winit::platform::web::WindowAttributesExtWebSys;

            let window = web_sys::window().expect("global `window` not found");
            let document = window.document().expect("`document` on window not found");
            let canvas = document
                .get_element_by_id("wasm-canvas")
                .expect("failed to find element with id 'wasm-canvas'")
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .expect("element with id 'wasm-canvas' is not an HtmlCanvasElement");

            attributes.with_canvas(Some(canvas))
        };

        let window = Arc::new(
            event_loop
                .create_window(attributes)
                .expect("failed to create winit window"),
        );
        self.window = Some(window.clone());
        window.request_redraw();

        let proxy = self.proxy.clone();
        let window_clone = window.clone();

        #[cfg(target_arch = "wasm32")]
        {
            wasm_bindgen_futures::spawn_local(async move {
                log::info!("⚙️ Step 1: Requesting GPU Adapter & Device...");
                let gpu = GpuContext::new(window_clone.clone()).await;
                log::info!("⚙️ Step 2: GPU Device ready! Building 3D pipeline...");
                let pipeline = ScenePipeline::new(
                    &gpu.device,
                    gpu.config.format,
                    Some(GpuContext::DEPTH_FORMAT),
                );
                log::info!("⚙️ Step 3: Pipeline built! Initializing UI Manager...");
                let ui = UiManager::new(&window_clone, &gpu.device, gpu.config.format);
                log::info!("⚙️ Step 4: UI Manager ready! Dispatching GpuReady event...");
                let _ = proxy.send_event(AppEvent::GpuReady { gpu, pipeline, ui });
            });
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let gpu = pollster::block_on(GpuContext::new(window_clone.clone()));
            let pipeline = ScenePipeline::new(
                &gpu.device,
                gpu.config.format,
                Some(GpuContext::DEPTH_FORMAT),
            );
            let ui = UiManager::new(&window_clone, &gpu.device, gpu.config.format);
            let _ = proxy.send_event(AppEvent::GpuReady { gpu, pipeline, ui });
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: AppEvent) {
        match event {
            AppEvent::GpuReady {
                mut gpu,
                pipeline,
                ui,
            } => {
                self.state.adapter_name = gpu.adapter_name.clone();
                self.state.adapter_backend = gpu.adapter_backend.clone();
                self.state.logs.push(
                    log::Level::Info,
                    format!(
                        "🎮 Adapter initialized: {} [{}]",
                        gpu.adapter_name, gpu.adapter_backend
                    ),
                );

                if let Some(window) = &self.window {
                    #[cfg(target_arch = "wasm32")]
                    let size = {
                        use winit::platform::web::WindowExtWebSys;
                        if let Some(canvas) = window.canvas() {
                            let dpr = window.scale_factor();
                            let width =
                                ((canvas.client_width() as f64) * dpr).round().max(1.0) as u32;
                            let height =
                                ((canvas.client_height() as f64) * dpr).round().max(1.0) as u32;
                            winit::dpi::PhysicalSize::new(width, height)
                        } else {
                            window.inner_size()
                        }
                    };

                    #[cfg(not(target_arch = "wasm32"))]
                    let size = window.inner_size();

                    if size.width > 0 && size.height > 0 {
                        gpu.resize(size);
                    }
                    window.request_redraw();
                }

                self.gpu = Some(gpu);
                self.pipeline = Some(pipeline);
                self.ui = Some(ui);
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        // Forward to Egui first
        let wants_pointer = if let (Some(ui), Some(window)) = (&mut self.ui, &self.window) {
            let response = ui.state.on_window_event(window, &event);
            if response.repaint {
                window.request_redraw();
            }
            if response.consumed {
                return;
            }
            ui.ctx.wants_pointer_input()
        } else {
            false
        };

        // Handle 3D Orbit Camera interaction if egui is not capturing the pointer
        match &event {
            WindowEvent::MouseInput {
                state,
                button: MouseButton::Left,
                ..
            } => {
                self.is_left_mouse_down = *state == ElementState::Pressed;
                if !self.is_left_mouse_down {
                    self.last_cursor_pos = None;
                }
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                let current_pos = [position.x as f32, position.y as f32];
                if self.is_left_mouse_down
                    && !wants_pointer
                    && let Some(last_pos) = self.last_cursor_pos
                {
                    let dx = current_pos[0] - last_pos[0];
                    let dy = current_pos[1] - last_pos[1];
                    self.camera.rotate(dx, dy);
                    if let Some(window) = &self.window {
                        window.request_redraw();
                    }
                }
                self.last_cursor_pos = Some(current_pos);
            }
            WindowEvent::MouseWheel { delta, .. } => {
                if !wants_pointer {
                    let scroll_amount = match delta {
                        MouseScrollDelta::LineDelta(_, y) => y * 40.0,
                        MouseScrollDelta::PixelDelta(pos) => pos.y as f32,
                    };
                    self.camera.zoom(scroll_amount);
                    if let Some(window) = &self.window {
                        window.request_redraw();
                    }
                }
            }
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                if let Some(gpu) = &mut self.gpu {
                    gpu.resize(*physical_size);
                }
            }
            WindowEvent::ScaleFactorChanged { .. } => {
                if let (Some(gpu), Some(window)) = (&mut self.gpu, &self.window) {
                    gpu.resize(window.inner_size());
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(window) = self.window.clone() {
                    match self.render_frame(&window) {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                            if let Some(gpu) = &mut self.gpu {
                                gpu.resize(gpu.size);
                            }
                        }
                        Err(wgpu::SurfaceError::OutOfMemory) => {
                            log::error!("GPU Out of Memory! Terminating event loop.");
                            event_loop.exit();
                        }
                        Err(e) => {
                            log::warn!("Render surface error: {e:?}");
                        }
                    }

                    // Handle CSD window actions after render
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        if self.should_close {
                            self.should_close = false;
                            event_loop.exit();
                            return;
                        }
                        if self.should_toggle_maximize {
                            self.should_toggle_maximize = false;
                            window.set_maximized(!window.is_maximized());
                        }
                        if self.should_drag_window {
                            self.should_drag_window = false;
                            let _ = window.drag_window();
                        }
                    }

                    // Adaptive frame pacing:
                    // If continuous animation (auto-rotate) is active or user is dragging the camera,
                    // keep redrawing at display refresh rate (60/120/144Hz).
                    // If static & idle, stop redraw loops to save 99% CPU/battery!
                    if self.state.transform.auto_rotate || self.is_left_mouse_down {
                        window.request_redraw();
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn run() {
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Debug).expect("failed to initialize logger");
        log::info!("🚀 Bootstrapping Professional Winit Event Loop...");

        use winit::platform::web::EventLoopExtWebSys;
        let event_loop = EventLoop::<AppEvent>::with_user_event()
            .build()
            .expect("failed to build event loop");
        let proxy = event_loop.create_proxy();
        let app = StudioApp::new(proxy);

        event_loop.spawn_app(app);
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
        log::info!("🚀 Bootstrapping Native Engine Studio...");

        let event_loop = EventLoop::<AppEvent>::with_user_event()
            .build()
            .expect("failed to build event loop");
        let proxy = event_loop.create_proxy();
        let mut app = StudioApp::new(proxy);
        let _ = event_loop.run_app(&mut app);
    }
}
