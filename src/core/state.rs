use glam::{Mat4, Quat, Vec3};
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeshType {
    HexagonPrism,
    Cube,
    Pyramid,
}

impl MeshType {
    pub const ALL: [Self; 3] = [Self::HexagonPrism, Self::Cube, Self::Pyramid];

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::HexagonPrism => "Hexagonal Prism",
            Self::Cube => "Smooth 3D Cube",
            Self::Pyramid => "Geometric Pyramid",
        }
    }
}

pub struct Transform {
    pub position: Vec3,
    pub rotation: Vec3, // Euler angles in radians (pitch, yaw, roll)
    pub scale: Vec3,
    pub auto_rotate: bool,
    pub rotation_speed: Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Vec3::ZERO,
            scale: Vec3::ONE,
            auto_rotate: true,
            rotation_speed: Vec3::new(0.4, 0.9, 0.2),
        }
    }
}

impl Transform {
    pub fn build_matrix(&self) -> Mat4 {
        let rotation_quat = Quat::from_euler(
            glam::EulerRot::YXZ,
            self.rotation.y,
            self.rotation.x,
            self.rotation.z,
        );
        Mat4::from_scale_rotation_translation(self.scale, rotation_quat, self.position)
    }

    pub fn update(&mut self, dt: f32) {
        if self.auto_rotate {
            self.rotation += self.rotation_speed * dt;
        }
    }
}

pub struct Material {
    pub color_tint: [f32; 4],
    pub ambient_strength: f32,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color_tint: [1.0, 1.0, 1.0, 1.0],
            ambient_strength: 0.35,
        }
    }
}

pub struct Environment {
    pub clear_color: [f32; 4],
    pub light_direction: Vec3,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            clear_color: [0.06, 0.08, 0.12, 1.0],
            light_direction: Vec3::new(0.5, -1.0, -0.6).normalize(),
        }
    }
}

pub struct UiLayout {
    pub show_menu_bar: bool,
    pub show_hierarchy: bool,
    pub show_inspector: bool,
    pub show_console: bool,
    pub show_stats_badge: bool,
}

impl Default for UiLayout {
    fn default() -> Self {
        Self {
            show_menu_bar: true,
            show_hierarchy: true,
            show_inspector: true,
            show_console: true,
            show_stats_badge: true,
        }
    }
}

#[derive(Clone)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: log::Level,
    pub message: String,
}

pub struct LogStore {
    pub entries: VecDeque<LogEntry>,
    pub max_entries: usize,
}

impl Default for LogStore {
    fn default() -> Self {
        let mut store = Self {
            entries: VecDeque::with_capacity(200),
            max_entries: 200,
        };
        store.push(log::Level::Info, "🚀 Engine studio subsystem started");
        store.push(log::Level::Info, "⚡ WebGPU Hardware Acceleration active");
        store.push(
            log::Level::Info,
            "👨‍💻 Developed by Rihad Jahan Opu • rihadjahanopu.com",
        );
        store
    }
}

impl LogStore {
    pub fn push(&mut self, level: log::Level, msg: impl Into<String>) {
        if self.entries.len() >= self.max_entries {
            self.entries.pop_front();
        }
        let now_sec = crate::core::time::TimeTracker::get_timestamp_secs();
        let timestamp = format!("{:.2}s", now_sec);
        self.entries.push_back(LogEntry {
            timestamp,
            level,
            message: msg.into(),
        });
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

/// Central state uniting 3D scene representation, camera target, UI visibility, and diagnostics.
pub struct AppState {
    pub mesh_type: MeshType,
    pub transform: Transform,
    pub material: Material,
    pub environment: Environment,
    pub ui_layout: UiLayout,
    pub logs: LogStore,
    pub adapter_name: String,
    pub adapter_backend: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            mesh_type: MeshType::HexagonPrism,
            transform: Transform::default(),
            material: Material::default(),
            environment: Environment::default(),
            ui_layout: UiLayout::default(),
            logs: LogStore::default(),
            adapter_name: "WebGPU Adapter".to_string(),
            adapter_backend: "BrowserWebGpu".to_string(),
        }
    }
}
