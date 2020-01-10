# Methods

Just like in C++ and Java, Rust has methods. Methods are functions that appear
to "live inside" the data they operate on.

```rust, editable
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    /// the `self` keyword is analogous to `this` in C++ and Javascript,
    /// or `self` in Objective-C. It means "the value this function was
    /// called on".
    ///
    /// Notice how we pass an immutable reference to the circle this
    /// function is called on. We could also pass a mutable reference
    /// which would allow the function to mutate the circle in-place.
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

fn main() {
    let c = Circle { x: 0.0, y: 0.0, radius: 2.0 };
    println!("{}", c.area());
}
```

[More info on method syntax](https://doc.rust-lang.org/1.8.0/book/method-syntax.html)
