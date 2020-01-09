# web-sys

[web-sys](https://crates.io/crates/web-sys) provides bindings for all Web APIs and
is a procedurally generated crate from WebIDL.

It's the defacto way to interact with a browser's DOM and other APIs.

It's part of a bigger project called
[wasm-bindgen](https://rustwasm.github.io/docs/wasm-bindgen/).

## A fair warning

Browser APIs are inherently un-safe. Any value in JavaScript may be `null`. As a
result strong, static typed languages have to jump through hoops to assure that
the programmer is working with the values they expect. In Rust this means doing a
lot of `map`ing and `unwrap`ing. The good news is that once you have a value, you
really have it.

## Comparisons

The `web-sys` crate is the low level crate that other Rust libraries and
frameworks build on top of. It is the bridge between Rust and your browser.

It's a bit like Haskell's [jsaddle](https://hackage.haskell.org/package/jsaddle)
and [jsaddle-dom](https://hackage.haskell.org/package/jsaddle-dom), which we use
at [Formation.ai](https://formation.ai/).

It's also a bit like interacting with your browser in JavaScript using the web
dev console. Very useful, but probably not the best way to structure large web
applications.
