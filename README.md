# Hello Rust

[Install](https://www.rust-lang.org/tools/install)

[Play ground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021)

[Rust book](https://doc.rust-lang.org/book/title-page.html)

[Rust by example](https://doc.rust-lang.org/rust-by-example/index.html)

```shell
# Verify installation
rustc --version
cargo --version

# Run
cargo run --bin hello

# Cloning this repo
git clone git@github.com:t4sk/hello-rust.git
cd hello-rust
cargo build
```

### Intro

- [x] Install cargo
- [x] [Hello world](./src/bin/hello.rs)
- [ ] [`println!`](./src/bin/print.rs)

### Data

- [ ] [Variable](./src/bin/variable.rs)
  - Immutable by default
  - `let`
  - `mut`
  - Constant
  - Shadowing
  - Type placeholder `_`
- [ ] [Scalar types](./src/bin/scalar.rs)
  - `i32`, `u32`, `f32`, `bool`, `char`
  - Type conversion
  - Min and max value
  - Integer overflow
- [ ] Compound data types
  - [ ] [Tuple](./src/bin/tuple.rs)
    - Destructure, `_`
    - Empty tuple
    - Nested
  - [ ] [Array](./src/bin/array.rs)
    - Array - collection of elements with length known at compile time
      - Fill
    - Slice - collection of elements with length not known at compile time
- [ ] [String](./src/bin/string.rs)
  - `r#`
  - `usize`
  - `format!`
- [ ] [Enum](./src/bin/enum.rs)
  - `Option`
  - `Result`
  - `derive(Debug, PartialEq)`
- [ ] [Struct](./src/bin/struct.rs)
  - Update syntax
  - [ ] [Struct method (`impl`)](./src/bin/struct_method.rs)
    - `Self`, `self`
    - Associated function (static method)
    - Methods (functions called on a particular instance of type)
- [ ] [Operators](./src/bin/operator.rs)
  - Math
  - Bool
  - Comparison
  - Bitwise

### Control flow

- [x] [if / else](./src/bin/if_else.rs)
- [x] [loop](./src/bin/loop.rs)
  - [x] for and range
  - [x] while
  - [x] returning values from loop
  - [x] `usize`
  - [x] label
- [x] [match](./src/bin/match.rs)
  - match on `Option` and `Result`
  - `_` to match rest
  - `@`
- [x] [if let and let else](./src/bin/if_let.rs)

### Function

- [x] [function](./src/bin/func.rs)
  - no return value
  - return value
  - implicit return
- [x] [mod](./src/bin/mods.rs)
  - visibility, `pub`
  - `use`, `super`
  - nested
  - struct
  - [ ] `crate`

### Error handling?

- [x] [error handling](./src/bin/error.rs)
  - [x] panic
  - [x] option
  - [x] result
- [x] [expect, unwrap](./src/bin/expect.rs)
- [x] [`?`](./src/bin/question.rs)
- [x] [`Box<dyn Error>`](./src/bin/box_dyn_error.rs)

### Ownership

- [x] [ownership](./src/bin/ownership.rs)
  - [x] stack and heap
    - stack (last in, first out)
      - store data with known fixed size
    - heap
      - vec (stack = vec pointer, data = heap)
  - Each value in Rust has an owner.
  - There can only be one owner at a time. (Transfer of ownership for variable assignment)
  - When the owner goes out of scope, the value will be dropped.
- [x] [borrow](./src/bin/borrow.rs)
  - immutable reference
  - mutable reference
  - cannot return ref
  - [x] [function](./src/bin/borrow_func.rs)
  - [x] [slice](./scr/bin/borrow_slice.rs)
    - slice is a reference to an collection so it's always borrowed
  - [x] [`String` and `str`](./src/bin/borrow_string_str.rs)
  - [x] [dereference](./src/bin/borrow_deref.rs)

### Standard collections

- [x] [vector](./src/bin/vec.rs)
- [x] [hash map](./src/bin/hash_map.rs)
- [x] [hash set](./src/bin/hash_set.rs)
