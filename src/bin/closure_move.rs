use std::thread;

fn main() {
    // Move
    // 3 ways values are captured from their environment
    // - borrowing immutable
    // - borrowing mutable
    // - taking ownership

    // borrow immutable
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let f_borrow_immut = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    f_borrow_immut();
    println!("After calling closure: {list:?}");

    // borrowing mutable
    println!("--- mutable borrow ---");

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    // Captures mutable ref
    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {list:?}");

    // taking ownership
    // move is useful for thread
    println!("--- move ---");

    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}
