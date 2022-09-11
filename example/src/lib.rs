use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add_one(a: u32) -> u32 {
    a + 1
}
