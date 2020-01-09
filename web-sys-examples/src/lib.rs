extern crate log;
extern crate console_log;
extern crate console_error_panic_hook;
extern crate serde;
extern crate serde_json;
extern crate web_sys;

use log::Level;
use std::panic;
use wasm_bindgen::prelude::*;
use web_sys::{
  window,
  Url
};

pub mod alert;
pub mod alert_button;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
pub fn main() -> Result<(), JsValue> {
  panic::set_hook(Box::new(console_error_panic_hook::hook));
  console_log::init_with_level(Level::Trace)
    .unwrap();

  let window = window().unwrap();
  let document =
    window
    .document()
    .unwrap();
  let location =
    document
    .location()
    .unwrap();
  let href:String = location.href()?;
  let url = Url::new(&href)?;
  let params = url.search_params();

  if params.has("alert") {
    alert::main()
  } else if params.has("alert_button") {
    alert_button::main()
  } else {
    Ok(())
  }
}
