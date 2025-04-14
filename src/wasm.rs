use wasm_bindgen::prelude::*;

#[wasm_bindgen(raw_module = "./assets/wasm_js.js")]
extern "C" {
    pub fn user_connected_wallet() -> String;
    pub fn mode() -> String;
}
