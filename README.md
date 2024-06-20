- [ ] install cargo
- [ ] println
- [ ] variable
  - immutable by default
  - `let`
  - type
  - `mut`
  - constant
  - shadowing
- [ ] scalar types
  - i32, u32, f32, boolean, char
  - integer overflow
- [ ] compound data types
  - array
    - create array with 100 elements
  - tuple
    - pattern matching
- [ ] function
  - no return value
  - return value
  - implicit return
- [ ] control flow
  - if, else if, else
  - if let
- [ ] loop
  - while let
- [ ] ownership
  - stack (last in, first out)
    - store data with known fixed size
  - heap
    - vec (stack = vec pointer, data = heap)
  - Each value in Rust has an owner.
  - There can only be one owner at a time.
  - When the owner goes out of scope, the value will be dropped.
- [ ] borrow
- [ ] struct
  - method
  - update syntax
- [ ] enum
- [ ] error handling
  - panic
  - option
  - result
  - expect, unwrap, ?
- [ ] String and str
- [ ] collection
  - slice
  - vector
  - hash map
- [ ] trait

# TODO:

https://github.com/wh5a/rustlings-solutions/tree/main/exercises

- string and str
- async
- ownership
  - clone
- borrow
  - reference
  - mutable reference
  - dreferencing `*`
  - slice
- lifetime
  - https://tfpk.github.io/lifetimekata/
  - static lifetime
  - anonymous lifetime
- destructing
  - struct
- option
  - Some(ref val)
- result
  - `Box<dyn Error>`
- pattern matching
- iterator
- map
- expect, unwrap
- mod
  - bin and lib crates
  - use, pub, super, use glob
- generics
- trait
  - impl A + B
- bounds
- smart pointer
- arc
- cow
- thread
- conversions
