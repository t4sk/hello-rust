#![allow(unused)]
// Closures are functions that can capture the enclosing environment
// - can save in a variable
// - can pass as function argument
fn main() {
    // Inference and annotation
    let val = 42;

    let annotated = |i: i32| -> i32 { i + val };
    let inferred = |i| i + val;

    println!("closure_annotated: {}", annotated(1));
    println!("closure_inferred: {}", inferred(1));

    // Infered type is locked to String -> String
    let id = |x| x;
    let s = id(String::from("hello"));

    // Closures capture value from environment in 3 ways
    // 1. borrowing immutable
    // 2. borrowing mutable
    // 3. taking ownership

    // Borrow immutable
    let color = String::from("red");
    let borrow_imut = || println!("color: {}", color);

    println!("color: {}", color);
    borrow_imut();
    println!("color: {}", color);

    // Borrow mutable
    let mut count = 0;
    let mut inc = || count += 1;

    inc();
    inc();
    inc();
    println!("count {}", count);

    // Taking ownership - use `move` to force ownership
    let list = vec![1, 2, 3];
    let f = std::thread::spawn(move || println!("list {:?}", list))
        .join()
        .unwrap();

    // This will not compile - ownership has moved inside the thread
    // println!("list {:?}", list);
}
