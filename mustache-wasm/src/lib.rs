mod utils;

use js_sys::Function;
use mustache_core::Value;
use wasm_bindgen::prelude::*;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

fn load_partial(partials: &Function, key: &str) -> Option<String> {
    partials
        .call1(&JsValue::NULL, &JsValue::from(key))
        .ok()
        .and_then(|value| value.as_string())
}

#[wasm_bindgen]
pub fn render(text: &str, data: &JsValue, partials: &Function) -> Result<String, JsValue> {
    let context: Value = data
        .into_serde()
        .map_err(|err| JsValue::from(err.to_string()))?;

    mustache_core::render(text, &context, |key| load_partial(partials, key))
        .map_err(|err| JsValue::from(&err))
}
