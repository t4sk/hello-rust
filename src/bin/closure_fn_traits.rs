#![allow(unused)]

fn f_once<F: FnOnce() -> T, T>(f: F) -> T {
    f()
    // Cannot call more than once
    // f()
}

fn f_mut<F: FnMut() -> T, T>(mut f: F) -> T {
    // Can be called more than once
    // f();
    f()
}

fn f_fn<F: Fn() -> T, T>(f: F) -> T {
    // Can be called more than once
    // f();
    f()
}

fn func() -> u32 {
    println!("func");
    32
}

fn main() {
    // FnOnce - closure can be called once
    //          moves captured values out of its body
    //        - uses captured value by value (T)
    // FnMut - closures that don't move captured values out of their body
    //         but might mutate the captured values
    //       - uses captured value by mut ref (&mut T)
    // Fn - closures that don't move captured values out of their body
    //      and don't mutate captured values
    //    - uses captured value by ref (&T)

    // FnOnce, FnMut, Fn used for function inputs and outputs

    // FnOnce
    let v = vec![1, 2, 3];
    let f = || {
        println!("fn once {:?}", v);
        v.len()
    };
    println!("before fn once {:?}", v);
    println!("fn once return: {}", f_once(f));
    println!("fn once return: {}", f_once(f));
    println!("after fn once {:?}", v);

    // FnMut
    let mut v = vec![1, 2, 3];
    // TODO: why mut keyword here?
    let mut f = || {
        v.push(4);
        v.len()
    };
    // TODO: why this doesn't compile
    // println!("before mut {:?}", v);
    println!("fn mut return: {}", f_mut(&mut f));
    println!("fn mut return: {}", f_mut(&mut f));
    println!("after fn mut {:?}", v);

    // Fn
    let v = vec![1, 2, 3];
    let f = || {
        println!("fn {:?}", v);
        v.len()
    };
    println!("before fn {:?}", v);
    println!("fn return: {}", f_fn(f));
    println!("fn return: {}", f_fn(f));
    println!("after fn {:?}", v);

    // normal function as input
    f_fn(func);
}
