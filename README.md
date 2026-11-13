# ⚡ WGPU 3D Engine Studio (WebAssembly + WebGPU + Rust)

[![Rust Edition](https://img.shields.io/badge/Rust-2024%20Edition-orange.svg)](https://www.rust-lang.org/)
[![WebGPU](https://img.shields.io/badge/Graphics-WebGPU%20%7C%20wgpu%2022-blue.svg)](https://wgpu.rs/)
[![WASM SIMD](https://img.shields.io/badge/WASM-SIMD128%20%2B%20Bulk%20Memory-brightgreen.svg)](https://webassembly.org/)
[![GUI](https://img.shields.io/badge/UI-egui%200.29%20(100%25%20GPU)-purple.svg)](https://github.com/emilk/egui)
[![Author](https://img.shields.io/badge/Developer-Rihad%20Jahan%20Opu-success.svg)](https://rihadjahanopu.com)

A professional Blender/Godot-style 3D WebGPU Engine Studio running entirely in the browser via Rust and WebAssembly. Features interactive 3D geometries, orbit camera controls, real-time uniform manipulation, and a multi-docked studio UI rendered 100% on the GPU with zero HTML DOM overhead.

---

## 👨‍💻 Developed by
**Rihad Jahan Opu**  
🌐 **Website:** [https://rihadjahanopu.com](https://rihadjahanopu.com)

---

## ✨ Features

- **⚡ Native WebGPU Rendering:** Pure modern graphics pipeline using `wgpu` and custom WGSL shaders with Blinn-Phong/Lambertian diffuse lighting and ambient tinting.
- **🚀 WebAssembly SIMD128:** Accelerated 128-bit vector arithmetic with `glam` math for ultra-fast transform and camera calculations.
- **🎨 100% GPU UI Rendering:** Docked studio interface rendered completely via VRAM font atlas and vertex quads using `egui` (zero DOM layout recalculation).
- **🧊 Multi-Mesh Studio Primitives:** Pre-baked Hexagonal Prism, Cube, and Pyramid geometries with zero GPU runtime allocation on mesh switching.
- **🎥 Interactive Orbit Camera:** Smooth spherical coordinate mouse orbit (Left-click drag) and wheel zoom.
- **🧠 Adaptive Frame Pacing:** Renders at full display refresh rate (60/120/144Hz) during animation and drops to 0.1% CPU in idle states to conserve battery.
- **🛠️ Studio Layout:**
  - **Top Menu Bar:** File, Geometry, Presets, Viewport controls, and developer branding.
  - **Left Hierarchy Tree:** Scene graph inspector and object nodes.
  - **Right Inspector:** Live transform controls, material tinting, lighting sliders, and GPU hardware device info.
  - **Bottom Console:** Live FPS sparkline history and diagnostic log stream.
  - **Floating 3D In-World Tag:** View-projection matrix projected 3D floating tag above the active mesh.

---

## 🏗️ Project Architecture

```
src/
├── lib.rs              # Library exports (app, core, gfx, ui)
├── main.rs             # Application entrypoint (WASM bindgen & Native)
├── app.rs              # ApplicationHandler, event loop, master render passes
├── core/
│   ├── mod.rs
│   ├── time.rs         # Frame delta tracking & FPS sparkline history
│   └── state.rs        # AppState, MeshType, Transform, Material, Logs
├── gfx/
│   ├── mod.rs
│   ├── context.rs      # GpuContext: surface, device, queue, depth buffer
│   ├── camera.rs       # OrbitCamera: view-projection matrix & spherical navigation
│   ├── mesh.rs         # Mesh generator: Hexagonal Prism, Cube, Pyramid
│   └── pipeline.rs     # Pre-baked GpuMesh buffers, RenderPipeline, WGSL uniforms
├── ui/
│   ├── mod.rs          # UiManager orchestrator
│   ├── theme.rs        # Studio dark theme
│   ├── menu_bar.rs     # Top studio menu bar
│   ├── hierarchy.rs    # Scene hierarchy panel
│   ├── inspector.rs    # Transform, material & hardware inspector
│   ├── console.rs      # Bottom frame latency graph & live logs
│   └── viewport.rs     # Viewport HUD & 3D projected floating mesh label
└── shaders/
    └── scene.wgsl      # WGSL vertex & fragment shaders
```

---

## 🛠️ Quick Start

### Prerequisites
- [Rust](https://rustup.rs/) (1.85+ with Rust 2024 edition support)
- `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- [Trunk](https://trunkrs.dev/): `cargo install trunk`

### Commands
```bash
# Start live development server (Hot Reloading at http://127.0.0.1:8080)
make dev

# Build optimized production release (SIMD128 + wasm-opt)
make release

# Run all code quality checks
make check

# Lint with Clippy
make lint

# Format codebase
make fmt
```

---

## 🌐 Deployment

The `dist/` folder is pre-configured with `_headers` containing Cross-Origin Isolation headers (`COOP`/`COEP`) required for WebGPU.

- **Netlify:** Drag & drop the `dist/` folder into Netlify.
- **GitHub Pages:** Push the `dist/` contents to a `gh-pages` branch.

---

## 📄 License
Created and maintained by **Rihad Jahan Opu** ([rihadjahanopu.com](https://rihadjahanopu.com)).
