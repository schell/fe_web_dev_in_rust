# Alert with button ⚙️

Now let's look at building some DOM.

First we'll need to add some more APIs as features to our Cargo.toml:

```toml
features = [
  "Document",
  "Element",
  "EventTarget",
  "HtmlElement",
  "Node",
  "Window",
]
```

Then we can write our app:

```rust, no_run, noplaypen
{{#include ../web-sys-examples/src/alert_button.rs}}
```

And see our work completed!

<iframe
  style="border:none"
  src="web-sys-examples/index.html?alert_button=true"
  width="100%">
</iframe>

That's an awful lot of code for a simple button that triggers an alert, but it's
fast and it's cautious.
