# -----------------------------------------------------------------------------
# ⚡ WGPU 3D Engine Studio — Automation Makefile
# Lead Developer: Rihad Jahan Opu <https://rihadjahanopu.com>
# -----------------------------------------------------------------------------

.DEFAULT_GOAL := help
SHELL := /bin/bash
PORT ?= 8080

.PHONY: help
help: ## Show this help message
	@echo "========================================================================"
	@echo "⚡ WGPU Engine Studio — Developer Command Center"
	@echo "👨‍💻 Developed by Rihad Jahan Opu • https://rihadjahanopu.com"
	@echo "========================================================================"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-16s\033[0m %s\n", $$1, $$2}'

.PHONY: dev
dev: ## Start live hot-reloading development server
	@echo "🚀 Starting Trunk development server on port $(PORT)..."
	trunk serve --port $(PORT)

.PHONY: serve
serve: ## Serve production release build with hot reload on port 8080
	@echo "⚡ Starting Trunk production release server on port $(PORT)..."
	trunk serve --release --port $(PORT)

.PHONY: build
build: ## Build debug WASM artifacts
	@echo "📦 Building debug WASM distribution..."
	trunk build
	@$(MAKE) deploy-prep

.PHONY: release
release: ## Build ultra-optimized production release (Fat LTO + SIMD128 + wasm-opt)
	@echo "🚀 Building production release distribution..."
	rm -rf dist
	trunk build --release
	@$(MAKE) deploy-prep
	@echo "✅ Production build finished successfully!"

.PHONY: desktop-run
desktop-run: ## Run the native 3D engine studio on your local desktop
	@echo "🖥️ Launching native desktop application..."
	cargo run --release

.PHONY: desktop-linux
desktop-linux: ## Build standalone Linux native executable
	@echo "🐧 Building Linux release binary..."
	cargo build --release
	@mkdir -p dist/desktop
	@cp target/release/wasm-wgpu-egui dist/desktop/wgpu-studio-linux-x86_64
	@echo "✅ Linux executable ready at dist/desktop/wgpu-studio-linux-x86_64"

.PHONY: deb
deb: desktop-linux ## Build standard Debian/Ubuntu .deb installer package
	@echo "📦 Packaging Debian/Ubuntu .deb installer..."
	@mkdir -p target/deb/DEBIAN target/deb/usr/bin target/deb/usr/share/applications dist/desktop
	@cp dist/desktop/wgpu-studio-linux-x86_64 target/deb/usr/bin/wgpu-studio
	@printf "Package: wgpu-studio\nVersion: 0.1.0\nSection: graphics\nPriority: optional\nArchitecture: amd64\nMaintainer: Rihad Jahan Opu <https://rihadjahanopu.com>\nDescription: WGPU Professional 3D Engine Studio\n High-performance cross-platform 3D WebGPU and egui studio shell.\n Developed by Rihad Jahan Opu (https://rihadjahanopu.com).\n" > target/deb/DEBIAN/control
	@printf "[Desktop Entry]\nName=WGPU Engine Studio\nComment=Professional 3D WebGPU Engine Studio by Rihad Jahan Opu\nExec=wgpu-studio\nIcon=applications-graphics\nTerminal=false\nType=Application\nCategories=Graphics;3DGraphics;Development;\n" > target/deb/usr/share/applications/wgpu-studio.desktop
	@dpkg-deb --build target/deb dist/desktop/wgpu-studio_0.1.0_amd64.deb
	@rm -rf target/deb
	@echo "✅ Debian/Ubuntu package ready at dist/desktop/wgpu-studio_0.1.0_amd64.deb"

.PHONY: appimage
appimage: desktop-linux ## Build standalone universal Linux .AppImage with .zsync / .rsync
	@echo "📦 Packaging universal Linux .AppImage..."
	@mkdir -p target/tools target/AppDir/usr/bin target/AppDir/usr/share/applications target/AppDir/usr/share/icons/hicolor/256x256/apps dist/desktop
	@if [ ! -f target/tools/squashfs-root/AppRun ]; then \
		echo "📥 Downloading appimagetool..."; \
		curl -sL https://github.com/AppImage/appimagetool/releases/download/continuous/appimagetool-x86_64.AppImage -o target/tools/appimagetool; \
		chmod +x target/tools/appimagetool; \
		(cd target/tools && ./appimagetool --appimage-extract > /dev/null); \
	fi
	@cp dist/desktop/wgpu-studio-linux-x86_64 target/AppDir/usr/bin/wgpu-studio
	@cp assets/wgpu-studio.png target/AppDir/wgpu-studio.png
	@cp assets/wgpu-studio.png target/AppDir/.DirIcon
	@cp assets/wgpu-studio.png target/AppDir/usr/share/icons/hicolor/256x256/apps/wgpu-studio.png
	@printf "[Desktop Entry]\nName=WGPU Engine Studio\nComment=⚡ Professional 3D WebGPU Engine Studio by Rihad Jahan Opu\nExec=wgpu-studio\nIcon=wgpu-studio\nTerminal=false\nType=Application\nCategories=Graphics;Development;\n" > target/AppDir/wgpu-studio.desktop
	@cp target/AppDir/wgpu-studio.desktop target/AppDir/usr/share/applications/
	@printf "#!/bin/sh\nSELF=\$$(readlink -f \"\$$0\")\nHERE=\$${SELF%%/*}\nexport PATH=\"\$${HERE}/usr/bin:\$${PATH}\"\nexec \"\$${HERE}/usr/bin/wgpu-studio\" \"\$$@\"\n" > target/AppDir/AppRun
	@chmod +x target/AppDir/AppRun
	@ARCH=x86_64 ./target/tools/squashfs-root/AppRun -u "gh-releases-zsync|rihad|wgpu-studio|latest|wgpu-studio-x86_64.AppImage.zsync" target/AppDir dist/desktop/wgpu-studio-x86_64.AppImage
	@if [ -f wgpu-studio-x86_64.AppImage.zsync ]; then \
		mv wgpu-studio-x86_64.AppImage.zsync dist/desktop/; \
		cp dist/desktop/wgpu-studio-x86_64.AppImage.zsync dist/desktop/wgpu-studio-x86_64.AppImage.rsync; \
	fi
	@echo "✅ AppImage bundle ready at dist/desktop/wgpu-studio-x86_64.AppImage"
	@echo "✅ AppImage delta update ready at dist/desktop/wgpu-studio-x86_64.AppImage.zsync & .rsync"

.PHONY: desktop-windows
desktop-windows: ## Cross-compile standalone Windows 64-bit .exe using MinGW
	@echo "🪟 Cross-compiling Windows 64-bit release .exe..."
	cargo build --release --target x86_64-pc-windows-gnu
	@mkdir -p dist/desktop
	@cp target/x86_64-pc-windows-gnu/release/wasm-wgpu-egui.exe dist/desktop/wgpu-studio-windows-x86_64.exe
	@echo "✅ Windows .exe ready at dist/desktop/wgpu-studio-windows-x86_64.exe"

.PHONY: all-platforms
all-platforms: release desktop-linux desktop-windows ## Build Web WASM, Linux, and Windows binaries simultaneously
	@echo "========================================================================"
	@echo "🎉 ALL CROSS-PLATFORM ARTIFACTS BUILT SUCCESSFULLY!"
	@echo "🌐 Web (WASM):     dist/ (index.html, wasm, js)"
	@echo "🐧 Linux Desktop:  dist/desktop/wgpu-studio-linux-x86_64"
	@echo "🪟 Windows Desktop: dist/desktop/wgpu-studio-windows-x86_64.exe"
	@echo "========================================================================"

.PHONY: check
check: check-native check-wasm ## Run compiler syntax & type checks on both Native & WASM targets
	@echo "✅ All platform checks passed!"

.PHONY: check-native
check-native: ## Check native target
	@echo "🔍 Checking native build target..."
	cargo check

.PHONY: check-wasm
check-wasm: ## Check wasm32 target with SIMD features
	@echo "🔍 Checking wasm32-unknown-unknown target..."
	cargo check --target wasm32-unknown-unknown

.PHONY: lint
lint: ## Run Clippy linter with strict studio rules
	@echo "🧹 Running Clippy lint analysis..."
	cargo clippy --target wasm32-unknown-unknown -- -D warnings || cargo clippy -- -D warnings

.PHONY: fmt
fmt: ## Format codebase according to rustfmt.toml
	@echo "🎨 Formatting codebase with rustfmt..."
	cargo fmt --all

.PHONY: fmt-check
fmt-check: ## Verify code formatting without making changes
	@echo "🔍 Checking codebase formatting..."
	cargo fmt --all -- --check

.PHONY: deploy-prep
deploy-prep: ## Ensure Netlify / GitHub Pages headers and configs exist in dist/
	@echo "🌐 Injecting WebGPU security headers into dist/_headers..."
	@mkdir -p dist
	@printf "/*\n  Cross-Origin-Opener-Policy: same-origin\n  Cross-Origin-Embedder-Policy: require-corp\n  Cross-Origin-Resource-Policy: cross-origin\n\n/*.wasm\n  Content-Type: application/wasm\n  Cache-Control: public, max-age=31536000, immutable\n\n/*.js\n  Cache-Control: public, max-age=31536000, immutable\n" > dist/_headers
	@echo "✅ dist/_headers ready for deployment."

.PHONY: clean
clean: ## Remove build artifacts, target directory, and dist folder
	@echo "🧹 Cleaning target and dist artifacts..."
	cargo clean
	rm -rf dist
	@echo "✨ Clean complete!"
