# Changelog

All notable changes to **WGPU Engine Studio** will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

---

## [0.4.0] — 2026-11-20

### ✨ Added

#### 🧊 New 3D Primitives
- **UV Sphere** — High-poly parametric sphere with smooth normals and vertex colors
- **Torus (Donut)** — 3D geometric torus with configurable major/minor radius
- **Cylinder** — Architectural cylinder with capped ends and smooth side normals
- **Subdivided Ground Plane** — Flat subdivided plane for ground/floor visualization

#### 🟦 Blender-Style 3D Grid Floor
- Stationary anti-aliased ground grid rendered with dedicated `grid_uniform_buffer` (identity matrix) so it stays fixed while the mesh rotates
- Color-coded coordinate axes: X-axis in **red**, Z-axis in **blue**
- Toggleable via UI checkbox or **`G`** keyboard shortcut

#### 💡 Blinn-Phong Specular Shading
- Full Blinn-Phong specular reflection model in WGSL (`scene.wgsl`)
- `camera_pos`, `specular_strength`, `light_color`, and `shininess` added to `Uniforms`
- World-space position passed from vertex to fragment shader (`world_pos`)
- Material `specular_strength` and `shininess` sliders exposed in Inspector
- Light color picker added to Environment section

#### ⌨️ Keyboard Shortcuts & Camera Presets
| Key | Action |
|-----|--------|
| `Space` | Toggle auto-rotation |
| `G` | Toggle 3D grid floor |
| `R` | Reset camera to default |
| `1` | Snap to Front view |
| `3` | Snap to Right view |
| `7` | Snap to Top view |

#### 🖥️ UI Enhancements
- Inspector: radio buttons for all 7 primitives (Hexagonal Prism, Cube, Pyramid, Sphere, Torus, Cylinder, Plane)
- Inspector: specular strength, shininess/glossiness sliders, light color picker
- Hierarchy: Camera Preset quick-buttons (Front, Right, Top, Reset)
- Hierarchy: 3D grid floor checkbox
- Menu Bar: View menu camera presets, Help menu keyboard shortcut cheatsheet

---

## [0.3.0] — 2026-11-19

### 🚀 CI & DevOps
- Migrated release automation to self-contained single-workflow architecture (inspired by gladeshell) with zero third-party token requirements.
- Added automatic git tagging directly within the release pipeline upon `Cargo.toml` version bump on `main`.
- Accelerated web WASM CI builds from 7 minutes to ~3 seconds using prebuilt trunk binary.
- Updated crate metadata with descriptive summary.

---

## [0.2.0] — 2026-11-13

### 🐛 Fixed
- `clippy.toml`: removed duplicate `cyclomatic-complexity-threshold` key (conflict with `cognitive-complexity-threshold`)
- `core/time.rs`: added `Default` impl for `TimeTracker` as required by clippy
- `app.rs`: collapsed nested `if` / `if let` into single condition

### ⚙️ CI
- Dependabot: blocked major version bumps (wgpu, egui, winit, glam) to prevent breaking CI
- Auto-tag workflow: creates git tag automatically on `Cargo.toml` version bump → triggers release
- WASM clippy: switched to `--lib` flag to avoid false-positive `main_recursion` error
- `Makefile`: `make build` now auto-generates `dist/_headers` for Netlify deploy

### 🎨 Style
- `menu_bar.rs`: applied rustfmt formatting

### 📝 Docs
- Added `CHANGELOG.md`

---

## [0.1.0] — 2026-11-13

> 🚀 Initial public release of WGPU Engine Studio — a cross-platform 3D engine built with WebGPU, egui, and Rust targeting both native desktop and WebAssembly.

### ✨ Added

#### Core Engine
- `lib.rs` — crate root with WASM entry point
- `main.rs` — native desktop entry point
- `app.rs` — main application loop (winit event loop, input handling, render dispatch)

#### Graphics (`gfx`)
- `context.rs` — WebGPU device, adapter, surface and queue initialization
- `camera.rs` — orbit camera with mouse drag and scroll zoom
- `mesh.rs` — 3D mesh primitives (cube, sphere, etc.) with vertex/index buffers
- `pipeline.rs` — WGSL render pipeline setup with depth buffer support

#### Shaders
- `scene.wgsl` — PBR-inspired vertex and fragment shaders for 3D scene rendering

#### Core Systems (`core`)
- `state.rs` — global engine state management
- `time.rs` — cross-platform high-precision frame timer and FPS tracker

#### UI (`ui` — egui panels)
- `theme.rs` — dark studio theme and style customization
- `menu_bar.rs` — top menu bar with File, View, Help menus
- `viewport.rs` — 3D viewport overlay and stats display
- `hierarchy.rs` — scene object hierarchy panel
- `inspector.rs` — object property inspector panel
- `console.rs` — developer console with log output

#### Web
- `index.html` — WebGPU-ready HTML entry point with loading screen and polyfills

#### Assets
- `wgpu-studio.png` / `wgpu-studio.svg` — application icon

### ⚙️ CI / DevOps
- GitHub Actions: Continuous Integration & Quality Gate (rustfmt, clippy, test matrix, WASM, docs)
- GitHub Actions: Cross-Platform Release (Linux, macOS, Windows, WebAssembly artifacts)
- GitHub Actions: GitHub Pages auto-deploy
- GitHub Actions: Security & License Audit
- GitHub Actions: Binary & WASM Size Benchmark
- GitHub Actions: Auto-tag on `Cargo.toml` version bump → triggers release automatically
- Dependabot: weekly dependency updates (patch/minor only — major bumps blocked)

### 🛠️ Configuration
- `Cargo.toml` — workspace with wgpu 22, egui 0.29, winit 0.30, glam 0.28
- `netlify.toml` + `dist/_headers` — WebGPU cross-origin isolation headers (COOP/COEP)
- `Makefile` — developer automation (dev, build, release, desktop, deb, AppImage, Windows)
- `clippy.toml` — strict lint rules
- `rustfmt.toml` — code style rules
- `.cargo/config.toml` — build target configuration

### 🐛 Fixed
- `clippy.toml`: removed duplicate `cyclomatic-complexity-threshold` key (conflict with `cognitive-complexity-threshold`)
- `core/time.rs`: added `Default` impl for `TimeTracker` as required by clippy
- `app.rs`: collapsed nested `if` / `if let` into single condition
- CI: WASM clippy now uses `--lib` to avoid false-positive `main_recursion` error
- CI: Dependabot blocked from opening major version bump PRs that break compilation

### 📦 Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `wgpu` | 22.0 | WebGPU graphics API |
| `winit` | 0.30 | Cross-platform windowing |
| `egui` | 0.29 | Immediate mode UI |
| `egui-wgpu` | 0.29 | egui ↔ wgpu integration |
| `egui-winit` | 0.29 | egui ↔ winit integration |
| `glam` | 0.28 | Math (Vec3, Mat4, etc.) |
| `bytemuck` | 1.16 | Safe GPU buffer casting |
| `wasm-bindgen` | 0.2 | WASM ↔ JS bridge |
| `web-sys` | 0.3 | Web APIs (Canvas, Window, etc.) |

---

[Unreleased]: https://github.com/rihadjahanopu/wasm-wgpu-egui/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/rihadjahanopu/wasm-wgpu-egui/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/rihadjahanopu/wasm-wgpu-egui/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/rihadjahanopu/wasm-wgpu-egui/releases/tag/v0.1.0
