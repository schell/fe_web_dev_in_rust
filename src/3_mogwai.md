# mogwai

Mogwai goes one step further than Gloo and provides patterns that make creating
DOM and interacting with it declarative and function. You still interact with
`web-sys`, but there are no event listeners or callbacks to deal with. It is
inspired by Haskell and Elm, so it looks a bit functional and components use a
familiar Elm update concept.

### Goals

* declarative DOM
* elm-like components
* explicit updates
* small (code size)
* fast

### Example

Here is an example of a button that counts its own clicks. Simple gizmos like
this don't need the elm-like component update scenario, so it looks very
functional:

```rust, ignore
extern crate mogwai;
use mogwai::prelude::*;

let (tx, rx) =
  txrx_fold(
    0,
    |n:&mut i32, _:&Event| -> String {
      *n += 1;
      if *n == 1 {
        "Clicked 1 time".to_string()
      } else {
        format!("Clicked {} times", *n)
      }
    }
  );

button()
  .style("cursor", "pointer")
  .rx_text("Clicked 0 times", rx)
  .tx_on("click", tx)
  .build().unwrap()
  .run().unwrap()
```

Here is an example of the same thing using an elm-like approach:

```rust, ignore
use mogwai::prelude::*;

pub struct Button {
  pub clicks: i32
}

#[derive(Clone)]
pub enum ButtonIn {
  Click
}

#[derive(Clone)]
pub enum ButtonOut {
  Clicks(i32)
}

impl Component for Button {
  type ModelMsg = ButtonIn;
  type ViewMsg = ButtonOut;

  fn update(
    &mut self,
    msg: &ButtonIn,
    tx_view: &Transmitter<ButtonOut>,
    _subscriber: &Subscriber<ButtonIn>
  ) {
    match msg {
      ButtonIn::Click => {
        self.clicks += 1;
        tx_view.send(&ButtonOut::Clicks(self.clicks))
      }
    }
  }

  fn builder(
    &self,
    tx: Transmitter<ButtonIn>,
    rx: Receiver<ButtonOut>
  ) -> GizmoBuilder {
    button()
      .style("cursor", "pointer")
      .rx_text("Clicked 0 times", rx.branch_map(|msg| {
        match msg {
          ButtonOut::Clicks(n) => format!("Clicked {} times", n)
        }
      }))
      .tx_on("click", tx.contra_map(|_| ButtonIn::Click))
  }
}

Button{ clicks: 0 }
  .into_component()
  .build().unwrap()
  .run().unwrap()
```
