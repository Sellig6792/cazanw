use wasm_bindgen::prelude::wasm_bindgen;

mod utils;

pub mod geometry;
pub mod image;
pub mod triangulation;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn init() {
    utils::set_panic_hook();
}
