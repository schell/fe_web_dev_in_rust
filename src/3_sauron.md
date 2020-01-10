# sauron

Sauron abstracts out `web-sys` completely in favor of a close clone of Elm. There
are no event listeners and callbacks.

## Example

```rust, ignore
use sauron::html::attributes::*;
use sauron::html::events::*;
use sauron::*;
use sauron::Component;
use sauron::Node;
use sauron::Program;
use sauron::Cmd;
use wasm_bindgen::prelude::*;
use log::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Msg {
    Click,
}

pub struct App {
    click_count: u32,
}

impl App {
    pub fn new() -> Self {
        App { click_count: 0 }
    }
}

impl Component<Msg> for App {

    fn view(&self) -> Node<Msg> {
        div!(
            [class("some-class"), id("some-id"), attr("data-id", 1)],
            [
                input!(
                    [
                        class("client"),
                        type_("button"),
                        value("Click me!"),
                        onclick(|_| {
                            trace!("Button is clicked");
                            Msg::Click
                        }),
                    ],
                    [],
                ),
                text!("Clicked: {}", self.click_count),
            ],
        )
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        trace!("App is updating from msg: {:?}", msg);
        match msg {
            Msg::Click => {
                self.click_count += 1;
                Cmd::none()
            }
        }
    }

}
```
