# Traits

Traits allow us to create an interface that types can implement (or not).

```rust
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

fn print_area<T: HasArea>(shape: T) {
    println!("This shape has an area of {}", shape.area());
}

fn main() {
  let c = Circle {
      x: 0.0f64,
      y: 0.0f64,
      radius: 1.0f64,
  };

  print_area(c);
}
```

[More info on traits](https://doc.rust-lang.org/1.8.0/book/traits.html)
