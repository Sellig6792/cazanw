//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

extern crate cazanw;
use cazanw::geometry::*;
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}


#[wasm_bindgen_test]
fn test_distance() {
    let a = Point { x: 0, y: 0 };
    let b = Point { x: 3, y: 4 };
    assert_eq!(distance(a, b), 5);
}
