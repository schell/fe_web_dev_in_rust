# Mutability
Mutable values are denoted using the `mut` keyword within a `let` binding, or in
the parameters of a function or pattern match (basically anywhere you can name a
value):
```rust, editable
fn main() {
  let mut name = "unused_name";
  name = "Cuco";
  print!("Hello {}", name);
}
```
