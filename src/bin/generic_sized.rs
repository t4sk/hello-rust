#![allow(unused)]

// Sized and ?Sized
// - Indicates that a type's size is known or not at compile time

// - Rust needs to know how much memory to allocate for any value
// - All values of a type must use the same amount of memory

// Dynamically sized types (DST)
// - Size is known only at runtime
// - Must always be behind a pointer

fn f<T: Sized>(x: T) {}
// This will not compile - T must be a pointer
// fn g<T: ?Sized>(x: T) {}
fn g<T: ?Sized>(x: &T) {}

fn main() {
    // Sized
    // - Trait where the type's size is known at compile time
    // - Automatically implemented for everything where the size is known at compile time
    // - Necessary for allocating values on the stack

    // Examples
    // Primitive types
    let i: i32 = 1;
    let x: f64 = 1.0;
    let b: bool = true;

    // Structs and enums with Sized fields
    struct S {
        i: i32,
        j: i32,
    };

    let s = S { i: 1, j: 1 };

    // Fixed size array
    let arr: [i32; 4] = [0; 4];

    f(i);
    f(s);
    f(arr);
    f(&arr);
    f("rust");

    // ?Sized
    // - Type may or may not be Sized
    // - Used for working with dynamically sized types (DST)

    // Examples
    // str and slices
    let s: &str = "hello";
    let slice: &[i32] = &[1, 2, 3];

    g(s);
    g(slice);

    // Trait objects
    let v: Box<dyn A> = Box::new(1u32);
    d(v);

    let v: Box<dyn A> = Box::new(1u32);
    g(&v);
}

trait A {}

impl A for u32 {}

fn d(x: Box<dyn A>) {}
