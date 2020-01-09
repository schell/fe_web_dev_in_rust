# Being Functional

## Lambdas!

Rust is a functional language in that it supports creating, storing and passing
anonymous functions as parameters to other functions.

```rust, editable
fn main() {
  let add_one =
    |x: &i32| {
      *x + 1
    };

  let add_one_mut =
    |x: &mut i32| {
      *x += 1
    };

  let mut n = add_one(&0);
  add_one_mut(&mut n);
}
```

## Options, folds, maps and more!

Through the use of [Traits](https://doc.rust-lang.org/1.8.0/book/traits.html) we
can use functions like `map`, `filter`, `fold` and friends on many different
structures.

Map the inner value of an `Option` type:

```rust
fn main() {
  let may_i32:Option<i32> = Some(88);
  let may_string:Option<String> =
    may_i32
    .map(|n| format!("{}", n));

  println!("{:?}", may_string);
}
```

Fold over a vector and sum the elements:

```rust
fn main() {
  let items:Vec<i32> = vec![0, 1, 2, 3];
  let sum =
    items
    .iter()
    .fold(
      0,
      |acc, i:&i32| acc + *i  // <- look, you don't always need brackets on lambdas
    );

  println!("The sum of {:?} is {}", items, sum);
}
```

Try a filter before the sum:

```rust
fn main() {
  let items:Vec<i32> = vec![0, 1, 2, 3];
  let sum =
    items
    .iter()
    .filter(|i| **i <= 2) // <- keep i if it is <= 2
    .fold(
      0,
      |acc, i| acc + *i
    );

  println!("The sum of numbers less that or equal to 2 in {:?} is {}", items, sum);
}
```

Do the same thing but with an imperative style:

```rust
fn main() {
  let items:Vec<i32> = vec![0, 1, 2, 3];

  let mut sum = 0;
  for i in items.iter() {
    if *i <= 2 {
      sum += i;
    }
  }

  println!("The sum of numbers less that or equal to 2 in {:?} is {}", items, sum);
}
```
