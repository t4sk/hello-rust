#![allow(unused)]

// Fn, FnMut and FnOnce traits

fn f_fn<T, F: Fn() -> T>(f: F) {
    // Can call more than once
    f();
    f();
}

fn f_mut<T, F: FnMut() -> T>(mut f: F) {
    // Can call more than once
    f();
    f();
}

fn f_once<T, F: FnOnce() -> T>(f: F) {
    // Cannot call more than once
    // f();
    f();
}

fn main() {
    // Fn (capture by &)
    // - immutable borrow from environment
    // - can be called more than once
    // FnMut (capture by &mut)
    // - mutable borrow from environment
    // - can be called more than once
    // FnOnce (capture by value)
    // - can be called at least once
    // - moves captured values into closure, if needed

    // Fn
    let s = "hello".to_string();
    let f = || println!("fn: {}", s);
    f_fn(f);
    // Can call more than once
    f_fn(f);
    println!("main: {}", s);

    // FnMut
    let mut v = vec![];
    let mut f = || v.push(0);
    f_mut(&mut f);
    // Can call more than once
    f_mut(&mut f);
    println!("main: {:?}", v);

    // FnOnce
    let v = vec![0, 1, 2];
    // Force transfer of v's ownership
    let f = move || println!("fn once: {:?}", v);
    f_once(f);
    // Cannot call more than once
    // f_once(f);
    // println!("main: {:?}", v);
}
