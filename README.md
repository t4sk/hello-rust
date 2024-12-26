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

- [ ] Install cargo
- [ ] [Hello world](./src/bin/hello.rs)
- [ ] [`println!`](./src/print.rs)

### Data

- [ ] [Variable](./src/variable.rs)
  - Immutable by default
  - `let`
  - `mut`
  - Constant
  - Shadowing
  - Type placeholder `_`
- [ ] [Scalar types](./src/scalar.rs)
  - `i32`, `u32`, `f32`, `bool`, `char`
  - Type conversion
  - Min and max value
  - Integer overflow
- [ ] Compound data types
  - [ ] [Tuple](./src/tuple.rs)
    - Destructure, `_`
    - Empty tuple
    - Nested
  - [ ] [Array](./src/array.rs)
    - Array - collection of elements with length known at compile time
      - Fill
    - Slice - collection of elements with length not known at compile time
- [ ] [String](./src/string.rs)
  - `r#`
  - `usize`
  - `format!`
- [ ] [Enum](./src/enum.rs)
  - `Option`
  - `Result`
  - `derive(Debug, PartialEq)`
- [ ] [Struct](./src/struct.rs)
  - Update syntax
  - [ ] [Struct method (`impl`)](./src/struct_method.rs)
    - `Self`, `self`
    - Associated function (static method)
    - Methods (functions called on a particular instance of type)
- [ ] [Operators](./src/operator.rs)
  - Math
  - Bool
  - Comparison
  - Bitwise
