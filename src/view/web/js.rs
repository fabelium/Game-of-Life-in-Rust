use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn console_log(s: &str);
}