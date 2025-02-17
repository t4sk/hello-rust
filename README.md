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
- [x] [Operators](./src/bin/operators.rs)
  - Math
  - Bool
  - Comparison
  - Bitwise

### Control flow

- [x] [if / else](./src/bin/if_else.rs)
- [x] [loop](./src/bin/loop.rs)
  - for and range
  - while
  - returning values from loop
  - `usize`
  - label
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

### Error handling

- [x] [error handling](./src/bin/error.rs)
  - panic
  - option
  - result
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
  - [ ] [slice](./src/bin/borrow_slice.rs)
    - slice is a reference to an collection so it's always borrowed
  - [ ] [`String` and `str`](./src/bin/borrow_string_str.rs)
  - [ ] [dereference](./src/bin/borrow_deref.rs)

### Standard collections

- [ ] [vector](./src/bin/vec.rs)
- [ ] [hash map](./src/bin/hash_map.rs)
- [ ] [hash set](./src/bin/hash_set.rs)

### Trait

- [ ] trait
  - [ ] [basic](./src/bin/trait_basic.rs)
    - `trait`, `impl`, default impl
  - [ ] common traits
    - [ ] [`derive`, `Debug`, `Default`, `PartialEq`, `Clone`](./src/bin/trait_common.rs)
    - [ ] [`Clone` and `Copy`](./src/bin/trait_clone_copy.rs)
    - [ ] [`Drop`](./src/bin/trait_drop.rs)
  - [ ] [trait fn input and output](./src/bin/trait_fn_io.rs)
  - [ ] [super trait `+`](./src/bin/trait_super.rs)
  - [ ] [fully qualified trait](./src/bin/trait_qualified.rs)

### Generic types

- [ ] generic types
  - basic
    - [ ] [struct, `Option`, `Result`, vector, `_`](./src/bin/generic_data.rs)
      - default G<T = A>
    - [ ] [function](./src/bin/generic_func.rs)
      - Monomorphization
  - [ ] [method for generic typed struct](./src/bin/generic_method.rs)
  - [ ] [generic traits](./src/bin/generic_trait.rs)
  - [ ] [`From` and `Into`](./src/bin/generic_from_into.rs)
  - [ ] [trait bound, `+`, `where`](./src/bin/generic_trait_bound.rs)
  - [ ] [`Sized` and `?Sized`](./src/bin/generic_sized.rs)
  - [ ] [dynamic dispatch](./src/bin/generic_dyn_dispatch.rs)
    - dynamic dispatch, `dyn`, `Box<dyn T>`
    - trait object
    - `Box<dyn Error>`
  - [ ] associated types
    - [ ] [basic](./src/bin/generic_assoc_type.rs)
    - [ ] [operator overloading](./src/bin/generic_op_overload.rs)
    - [ ] [iterator](./src/bin/generic_iter.rs)
      - vec, array, hashmap
      - counter
      - `iter` and `into_iter`, `iter_mut`
    - [ ] [iterator adaptors `map`, `filter`, `collect`, `fold`, `enumerate`, `rev`, `zip`](./src/bin/generic_iter_adaptor.rs)
    - [ ] [while let](./src/bin/while_let.rs)
  - [ ] [lifetimes](./src/bin/generic_lifetime.rs)
    - elision
    - static
    - `'_`

### Closure

- [ ] [Function pointer](./src/bin/fn_pointer.rs)
- [ ] [Basic](./src/bin/closure.rs)
  - `map` example
  - difference between func pointer
- [ ] [Borrow, ownership and `move`](./src/bin/move.rs)
- [ ] [`Fn`, `FnMut`, `FnOnce`](./src/bin/fn_traits.rs)
- [ ] [Closure as input and output](./src/bin/closure_out.rs)

### Concurrency

- [ ] [thread](./src/bin/thread.rs)
- [ ] [thread `move`](./src/bin/thread_move.rs)
