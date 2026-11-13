# Changelog

All notable changes to **WGPU Engine Studio** will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

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

[Unreleased]: https://github.com/rihadjahanopu/wasm-wgpu-egui/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/rihadjahanopu/wasm-wgpu-egui/releases/tag/v0.1.0
