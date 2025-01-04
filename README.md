# Hello Rust

[Install](https://www.rust-lang.org/tools/install)

[Play ground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021)

[Rust book](https://doc.rust-lang.org/book/title-page.html)

[Rust by example](https://doc.rust-lang.org/rust-by-example/index.html)

[Rust reference](https://doc.rust-lang.org/reference/introduction.html)

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
- [x] [`println!`](./src/bin/print.rs)

### Data

- [x] [Variable](./src/bin/variable.rs)
  - Immutable by default
  - `let`
  - `mut`
  - Constant
  - Shadowing
  - Type placeholder `_`
- [x] [Scalar types](./src/bin/scalar.rs)
  - `i32`, `u32`, `f32`, `bool`, `char`
  - Type conversion
  - Min and max value
  - Integer overflow
- [x] Compound data types
  - [x] [Tuple](./src/bin/tuple.rs)
    - Destructure, `_`
    - Empty tuple
    - Nested
  - [x] [Array](./src/bin/array.rs)
    - Array - collection of elements with length known at compile time
      - Fill
    - Slice - collection of elements with length not known at compile time
- [x] [String](./src/bin/string.rs)
  - `r#`
  - `usize`
  - `format!`
- [x] [Enum](./src/bin/enum.rs)
  - `Option`
  - `Result`
  - `derive(Debug, PartialEq)`
- [x] [Struct](./src/bin/struct.rs)
  - Update syntax
  - [x] [Struct method (`impl`)](./src/bin/struct_method.rs)
    - `Self`, `self`
    - Associated function (static method)
    - Methods (functions called on a particular instance of type)
- [ ] [Operators](./src/bin/operators.rs)
  - Math
  - Bool
  - Comparison
  - Bitwise

### Control flow

- [ ] [if / else](./src/bin/if_else.rs)
- [ ] [loop](./src/bin/loop.rs)
  - [ ] for and range
  - [ ] while
  - [ ] returning values from loop
  - [ ] `usize`
  - [ ] label
- [ ] [match](./src/bin/match.rs)
  - match on `Option` and `Result`
  - `_` to match rest
  - `@`
- [ ] [if let and let else](./src/bin/if_let.rs)

### Function

- [ ] [function](./src/bin/func.rs)
  - no return value
  - return value
  - implicit return
- [ ] [mod](./src/bin/mods.rs)
  - visibility, `pub`
  - `use`, `super`
  - nested
  - struct
  - [ ] `crate`

### Error handling?

- [ ] [error handling](./src/bin/error.rs)
  - [ ] panic
  - [ ] option
  - [ ] result
- [ ] [expect, unwrap](./src/bin/expect.rs)
- [ ] [`?`](./src/bin/question.rs)
- [ ] [`Box<dyn Error>`](./src/bin/box_dyn_error.rs)

### Ownership

- [ ] [ownership](./src/bin/ownership.rs)
  - [ ] stack and heap
    - stack (last in, first out)
      - store data with known fixed size
    - heap
      - vec (stack = vec pointer, data = heap)
  - Each value in Rust has an owner.
  - There can only be one owner at a time. (Transfer of ownership for variable assignment)
  - When the owner goes out of scope, the value will be dropped.
- [ ] [borrow](./src/bin/borrow.rs)
  - immutable reference
  - mutable reference
  - cannot return ref
  - [ ] [function](./src/bin/borrow_func.rs)
  - [ ] [slice](./scr/bin/borrow_slice.rs)
    - slice is a reference to an collection so it's always borrowed
  - [ ] [`String` and `str`](./src/bin/borrow_string_str.rs)
  - [ ] [dereference](./src/bin/borrow_deref.rs)

### Standard collections

- [ ] [vector](./src/bin/vec.rs)
- [ ] [hash map](./src/bin/hash_map.rs)
- [ ] [hash set](./src/bin/hash_set.rs)
