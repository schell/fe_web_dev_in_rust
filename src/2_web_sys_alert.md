# Alert 🚨

Lets write an alert.

## A note on using web-sys APIs

We can find the API we need using a
[search](https://rustwasm.github.io/wasm-bindgen/api/web_sys/?search=alert)
within the web-sys docs:

> `pub fn alert_with_message(&self, message: &str) -> Result<(), JsValue>`
>
> The alert() method
>
> MDN Documentation
>
> _This API requires the following crate features to be activated: Window_

This last bit about "This API require the following..." is important. `web-sys`
is huuuuuge because it contains a lot of APIs. To cut down on compile times, etc.
the authors decided to have each API be *opt-in*, so you must add that API to your
`Cargo.toml`'s features list. Luckily we have a project template that includes
this stuff (along with a little note):

```toml
[dependencies.web-sys]
version = "0.3"
# Add more web-sys API's as you need them
features = [
  "HtmlInputElement",
]
```

To use the `Window` API we have to add "Window" to the list of features:

```toml
features = [
  "HtmlInputElement",
  "Window"
]
```

Now we can write our little app:

```rust, no_run, noplaypen
{{#include ../web-sys-examples/src/alert.rs}}
```
