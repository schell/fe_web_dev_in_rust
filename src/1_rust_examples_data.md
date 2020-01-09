# Data Types

Rust has two kinds of data structures.

## Struct

Like Haskell's records, without the record field update baggage.

```rust
struct Person {
  name: String
}

struct Place {
  name: String,
  latitude: f32,
  longitude: f32,
}

struct Thing {
  description: String
}
```

## Enum

Enums can be C-like:

```rust
enum Noun {
  Person,
  Place,
  Thing
}
```

Or they can be more like Haskell's sum types:

```rust, editable
#[derive(Debug)]
struct Person {
  name: String
}

#[derive(Debug)]
struct Place {
  name: String,
  latitude: f32,
  longitude: f32,
}

#[derive(Debug)]
struct Thing {
  description: String
}

#[derive(Debug)]
enum Noun {
  Person(Person),
  Place(Place),
  Thing(Thing)
}

fn main() {
  let thing =
    Thing {
      description: "A broken can opener".to_string()
    };
  let mut noun =
    Noun::Thing(thing);

  println!("{:#?}", noun);

  match &mut noun {
    Noun::Thing(thing) => {
      thing.description = "A can't opener".to_string()
    }
    _ => {}
  }

  println!("{:#?}", noun);
}
```
