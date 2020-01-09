# Rust in the Browser

Rust cross-compiles to [WebAssembly](https://webassembly.org/).

You'll need [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) to
convert your Rust into WASM and to shim it into an html file.

Once you have `wasm-pack` you can use `cargo generate` (which can be installed
with `cargo install cargo-generate`) to create a template project:

```bash
cargo generate --git https://github.com/schell/mogwai-template.git
```

The `cd` into your new project and:

```bash
wasm-pack build --target no-modules
```

Then, if you don't already have it, `cargo install basic-http-server` or use your
favorite alternative to serve the files for your app:

```bash
basic-http-server -a 127.0.0.1:8888
```

With `wasm-pack` you'll be able to create Rust packages that can be consumed and
published with `npm`.
