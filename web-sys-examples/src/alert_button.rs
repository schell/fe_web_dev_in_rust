use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
  window,
  Element,
  HtmlElement,
  Node
};

pub fn main() -> Result<(), JsValue> {
  // Get a handle to our window and document
  let window = window().unwrap();
  let document =
    window
    .document()
    .unwrap();

  // Create the button element
  let button:Element =
    document
    .create_element("button")
    .unwrap();
  button
    .dyn_ref::<HtmlElement>()
    .unwrap()
    .set_inner_text("Trigger");

  // Get a handle on the document's body
  let body =
    document
    .body()
    .unwrap();

  // Add the button to the DOM
  body
    .dyn_into::<Node>()
    .unwrap()
    .append_child(&button)
    .unwrap();

  // Create a Javascript closure that will trigger an alert
  let cb =
    Closure::wrap(Box::new(|| {
      web_sys::window()
        .unwrap()
        .alert_with_message("You hit the button!")
        .unwrap();
    }) as Box<dyn FnMut()>);

  // Add the closure as a listener that procs on click
  button
    .add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())
    .unwrap();

  // Usually Rust would hold on to the closure (or else it is dropped)
  // but here we'll forget about it, which means it will be around forever.
  cb.forget();

  Ok(())
}
