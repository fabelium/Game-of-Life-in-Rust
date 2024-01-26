use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::js_sys::Promise;
use wasm_bindgen_futures::{js_sys, JsFuture};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn console_log(s: &str);

    #[wasm_bindgen(js_namespace = window)]
    pub fn fetch(input: &str) -> js_sys::Promise;
}

#[wasm_bindgen]
pub async fn load_pattern_in_rust(pattern_filename: &str) {
    console_log(("Loading pattern ".to_owned() + pattern_filename).as_str());
    match make_fetch_call(pattern_filename).await {
        Ok(text) => {
            console_log(&text);
            let pattern: Vec<String> = text.lines().map(|line| line.to_string()).collect();
            let cell_size: u8 = 10;
            crate::initialize_game(Some(pattern), cell_size);
        }
        Err(err) => {
            console_log(err.as_string().unwrap().as_str());
        }
    }
}

pub async fn make_fetch_call(url: &str) -> Result<String, JsValue> {
    let response_promise: Promise = web_sys::window()
        .unwrap()
        .fetch_with_str(url);

    let response = JsFuture::from(response_promise)
        .await
        .map_err(|err| {
            console_log(&format!("Fetch error: {:?}", err));
            JsValue::from_str("Fetch error")
        })?
        .dyn_into::<web_sys::Response>()
        .map_err(|err| {
            console_log(&format!("Response conversion error: {:?}", err));
            JsValue::from_str("Response conversion error")
        })?;

    let text = JsFuture::from(response.text()?)
        .await
        .map(|text| text.as_string().unwrap_or_else(|| "".to_string()))
        .map_err(|err| {
            console_log(&format!("Text conversion error: {:?}", err));
            JsValue::from_str("Text conversion error")
        })?;

    Ok(text)
}
