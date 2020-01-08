# References
Like C and C++, Rust supports passing values by value or by reference. A
reference is a pointer to a value. Unlike those languages, passing a value
"invokes" the borrow checker. The borrow checker ensures that no more than one
mutable reference to a value is obtained at any one
time:
```rust, editable
fn add_one_mut(x: &mut i32) {
  // Here we dereference the variable to add one to its value
  *x += 1;
}

fn main() {
  let mut x = 6;
  add_one_mut(&mut x); // <- see that we're passing a mutable reference
  add_one_mut(&mut x);
  println!("Hello {}", x);
}
```

Here we'll try to hold one mutable reference and pass another at the same time:
```rust, editable
fn add_one_mut(x: &mut i32) {
  *x += 1;
}

fn main() {
  let mut x = 6;
  let may_mut_x:Option<&mut i32> = Some(&mut x);
  add_one_mut(&mut x);
  println!("Hello {:?}", may_mut_x);
}
```

But you may have as many immutable references as you like, so long as none of
those references try to "outlive" the original value.

```rust, editable
/// Returns one plus the value of the reference passed
fn add_one(x: &i32) -> i32 {
  *x + 1
}

fn main() {
  let x = 3;
  let may_x_ref = Some(&x);
  let y = add_one(&x);
  let z = add_one(&y);
  println!("Hello {} {} {} {:?}", x, y, z, may_x_ref);
}
```
