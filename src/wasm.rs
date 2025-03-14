use wasm_bindgen::prelude::*;

#[wasm_bindgen(raw_module = "./assets/wasm_js.js")]
extern "C" {
    pub fn test(uuid: String);
}
