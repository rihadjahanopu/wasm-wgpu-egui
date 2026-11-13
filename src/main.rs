#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

fn main() {
    wasm_wgpu_egui::run();
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn run_wasm() {
    main();
}
