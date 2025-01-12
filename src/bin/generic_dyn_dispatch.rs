#![allow(unused)]

// Static and dynamic dispatch

// Static dispatch
// - function to call is known at compile time
// - monomorphization (code size can be larger)
// - no run time cost (no vtable lookup)
// Dynamic dispatch
// - function to call is known at run time
// - vtable lookup (code size is smaller)
// - run time overhead (vtable lookup)

// Trait object
// Value that implements a trait, but its concrete type is unknown at compile time

#[derive(Debug)]
struct A;

#[derive(Debug)]
struct B;

trait F {
    fn f(&self);
}

impl F for A {
    fn f(&self) {
        println!("{:?}", self)
    }
}

impl F for B {
    fn f(&self) {
        println!("{:?}", self)
    }
}

// Static dispatch
// T must resolve to a concrete type at compile time
fn static_dispatch<T: F>(t: &T) {
    t.f();
}

// Dynamic dispatch
// t is known at run time
// &dyn F is a reference to a trait object
// dyn - used to create a trait object
// Why &?
// - pointer to a type that implements F
// - trait obj (dyn F) is unsized
fn dyn_dispatch(t: &dyn F) {
    t.f();
}

// Difference between &dyn F and Box<dyn F>
// &dyn F - borrow
// Box<dyn F> takes ownership
fn dyn_dispatch_box(t: Box<dyn F>) {
    t.f();
}

fn main() {
    let obj = A;
    static_dispatch(&obj);
    let obj = B;
    static_dispatch(&obj);

    // user input
    let input = "A";

    let obj: &dyn F = match input {
        "A" => &A,
        "B" => &B,
        _ => panic!(),
    };

    // This won't compile
    // Type must be known at compile time
    // static_dispatch(obj);

    dyn_dispatch(obj);

    // Box<dyn T>
    // Allocates memory on the heap, stores concrete instance
    let obj: Box<dyn F> = match input {
        "A" => Box::new(A),
        "B" => Box::new(B),
        _ => panic!(),
    };

    dyn_dispatch_box(obj);
    // This won't compile
    // dyn_dispatch_box takes ownership
    // let x = obj;
}
