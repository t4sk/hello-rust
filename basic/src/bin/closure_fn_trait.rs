fn f_once<F: FnOnce()>(f: F) {
    f();
}

fn f_mut<F: FnMut()>(mut f: F) {
    f();
}

fn f_fn<F: Fn()>(f: F) {
    f();
}

fn func() {
    println!("func");
}

fn main() {
    // Needed for function inputs and outputs
    // FnOnce
    // applies to closures that can be called once.
    // All closures implement at least this trait, because all closures can be called.
    // A closure that moves captured values out of its body will only implement FnOnce and none of the other Fn traits, because it can only be called once.

    // FnMut
    // applies to closures that don’t move captured values out of their body, but that might mutate the captured values. These closures can be called more than once.

    // Fn
    // applies to closures that don’t move captured values out of their body
    // and that don’t mutate captured values,
    // as well as closures that capture nothing from their environment.
    // These closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times concurrently.

    // fn, dyn fn, Fn, etc...

    // FnOnce
    let v = vec![1, 2, 3];
    let f = || println!("once {:?}", v);
    println!("before once {:?}", v);
    f_once(f);
    f_once(f);
    println!("after once {:?}", v);

    // FnMut
    let mut v = vec![1, 2, 3];
    // TODO: why mut keyword here?
    let mut f = || v.push(4);
    // TODO: why this doesn't compile
    // println!("before mut {:?}", v);
    f_mut(&mut f);
    f_mut(&mut f);
    println!("after mut {:?}", v);

    // Fn
    let v = vec![1, 2, 3];
    let f = || println!("v {:?}", v);
    println!("before fn {:?}", v);
    f_fn(f);
    f_fn(f);
    println!("after fn {:?}", v);

    // normal function as input
    f_fn(func);
}
