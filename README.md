- [ ] install cargo
- [x] hello world
- [x] variable
  - immutable by default
  - `let`
  - type
  - `mut`
  - constant
  - shadowing
- [x] scalar types
  - i32, u32, f32, boolean, char
  - integer overflow
  - type conversion
- [x] compound data types
  - array
    - array - collection of elements with length known at compile time
    - slice - collection of elements with length known at runtime
  - tuple
    - pattern matching
- [x] function
  - no return value
  - return value
  - implicit return
- [x] control flow
  - [x] if / else
  - [x] match
  - [x] if let
  - [x] let else
- [x] loop
  - [x] for and range
  - [x] while
  - [x] while let
  - [x] returning values from loop
- [x] returning values from block expression
- [ ] ownership
  - stack (last in, first out)
    - store data with known fixed size
  - heap
    - vec (stack = vec pointer, data = heap)
  - Each value in Rust has an owner.
  - There can only be one owner at a time. (Transfer of ownership)
  - When the owner goes out of scope, the value will be dropped.
  - passing variable into function
- [x] borrow
  - reference
  - mutable reference
  - no dangling refe
- [x] slice
- [x] string and str
  - str - string with length known at runtime
  - `r#`
- [x] struct
  - [x] update syntax
  - [x] method (impl)
  - [x] derive
- [x] enum
- [ ] error handling
  - panic
  - option
  - result
  - ?
  - box dyn error
  - [ ] expect, unwrap
- [x] generic types
  - `impl <T> MyStruct<T>`
- [ ] trait
  - [x] default
  - [x] trait fn input
  - [x] trait bound, `+`, `where`
  - [ ] trait object
  - [ ] return impl from func
  - [x] From and Into
  - [x] Add and Mul
  - [ ] fmt::Display
- [x] lifetimes
  - static
- [ ] closures
  - [ ] fn traits (Fn, FnMut and FnOnce) (TODO)
  - where syntax, dynamic dispatch
  - difference between func pointer and fn traits and closure
- [x] iterators
  - [ ] iter - doesn't take ownership (iterate by reference)
  - [ ] into_iter - takes ownership (moves items into new scope)
  - [x] map, filter, filter_map, fold, flatten
  - sort_by ?
- [ ] collection
  - vector
  - [ ] slice
  - hash map
  - [x] hash set
- [ ] mod
- [ ] smart pointers
  - [x] Box
  - [x] Rc
  - [ ] arc
- associated types

- `dbg, println!("{:#?}")`
- default
- scope
- concurrency
  - thread
  - channel
  - mutex
  - async
- dereference
- `Box<dyn Error>`
- cow
- `ref mut`
- `todo!()`
- thread scope

https://github.com/wh5a/rustlings-solutions/tree/main/exercises

# app

- [x] secant method
- [x] create address
- [x] create2 address
- [x] merkle proof
- [ ] mpt proof
- [x] FFT
- zkp
- [x] spinner
- [ ] anzan
- real time code sharing editor
