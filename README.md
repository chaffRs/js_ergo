# js_ergo

Ergonomic, JavaScript-style string helpers for Rust.

`js_ergo` brings the convenience of JavaScript's `String` methods to Rust as a
zero-dependency extension trait, while keeping idiomatic, Unicode-correct
semantics.

## Install

```toml
[dependencies]
js_ergo = "0.1"
```

## Usage

Bring the [`JsStrExt`] trait into scope and call the helpers on any `&str`:

```rust
use js_ergo::JsStrExt;

// Pad the start with a single character.
assert_eq!("123".pad_start(5, '0'), "00123");

// Pad the end, too.
assert_eq!("123".pad_end(5, '0'), "12300");

// A multi-character pad is repeated and truncated (like JS `padStart`).
assert_eq!("5".pad_start(4, "ab"), "aba5");

// Already long enough? Returned unchanged.
assert_eq!("hello".pad_start(3, '.'), "hello");
```

The pad argument accepts a `char`, `&str`, `&String`, or `String`. A
multi-character pad is repeated and truncated to fill the gap.

## Note on length

`length` is counted in Unicode scalar values (`char`s), **not** UTF-16 code
units as in JavaScript. A character outside the Basic Multilingual Plane such
as `'🦀'` counts as 1 here but as 2 in JavaScript.

## Minimum supported Rust version

Rust 1.85 (edition 2024).

## License

Licensed under the [MIT License](LICENSE).

[`JsStrExt`]: https://docs.rs/js_ergo/latest/js_ergo/trait.JsStrExt.html
