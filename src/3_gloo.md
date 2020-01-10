# Gloo

Gloo is a library that adds some much needed clean-up on top of `web-sys`. It
makes writing `web-sys` code look a lot like writing regular javascript, only in
Rust!

### Goals

* fast
* reliable
* small (code size)
* idiomatic

### Example

```rust, ignore
use gloo::{events::EventListener, timers::callback::Timeout};
use wasm_bindgen::prelude::*;

pub struct DelayedHelloButton {
    button: web_sys::Element,
    on_click: events::EventListener,
}

impl DelayedHelloButton {
    pub fn new(document: &web_sys::Document) -> Result<DelayedHelloButton, JsValue> {
        // Create a `<button>` element.
        let button = document.create_element("button")?;

        // Listen to "click" events on the button.
        let button2 = button.clone();
        let on_click = EventListener::new(&button, "click", move |_event| {
            // After a one second timeout, update the button's text content.
            let button3 = button2.clone();
            Timeout::new(1_000, move || {
                button3.set_text_content(Some("Hello from one second ago!"));
            })
            .forget();
        });

        Ok(DelayedHelloButton { button, on_click })
    }
}
```
