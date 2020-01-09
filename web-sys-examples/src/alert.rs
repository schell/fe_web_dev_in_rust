use wasm_bindgen::prelude::*;
use web_sys::window;

pub fn main() -> Result<(), JsValue> {
  let window = window().unwrap();
  window.alert_with_message("Hello from WASM!")
}
