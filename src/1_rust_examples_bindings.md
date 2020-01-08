# Functions, Values and Bindings
Rust is C-like, with braces and a mostly imperative style. Here's hello world:
```rust, editable
fn main() {
  let name = "world";
  print!("Hello {}", name);
}
```

`let` bindings allow us to bind names to values, and names may be rebound:
```rust, editable
fn main() {
  let x:f32 = 1.0;
  let y:f32 = 2.5;
  let name:f32 = x + y;
  let name:String = format!("Number {:.2}", name);
  print!("Hello {}", name);
}
```
