# Rust's Features

- compiled w/ the `rustc` compiler, which is self-hosted (written in Rust)

- a strong, static typing discipline, similar in feel to Haskell 98 but different

- imperative or functional style, a bit like Scala and Swift. You can mix'n'match
  your functional and imperative code - but EVERYTHING IS IN IO. (Opinion): this
  is not bad.

- [eager evaluation](https://en.m.wikipedia.org/wiki/Eager_evaluation) of
  expressions.

- type variables or [generics](https://doc.rust-lang.org/rust-by-example/generics.html)

- [traits](https://en.m.wikipedia.org/wiki/Trait_(computer_programming))
  (which are a bit like interfaces or type classes, less like classes but you get
  the point)

- a new, novel concept called a "borrow checker", which obviates garbage
  collection (more on this later, because it is a big deal). It makes memory
  management rather slick and easy.

- [macros!](https://doc.rust-lang.org/rust-by-example/macros.html) The macro
  system provides compile-time metaprogramming much like TemplateHaskell, but
  unlike the Haskell community, the Rust community seems very inclined to use
  macros whenever they feel like it (I personally like this)

## Rust's Non-features

- No higher-kinded types ... (Opinion): this means there are no practical
  implementations of category theory concepts like Category, Arrow, Functor,
  Monoid, Monad, etc.
  This is a good thing, _mostly_.

- No classical inheritance (and no prototypal inheritance, for that matter). This
  is good.
