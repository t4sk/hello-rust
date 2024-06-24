fn print_msg(msg: &String) {
    println!("Message is {}", msg);
}

fn add_to_vec(v: &mut Vec<i32>) {
    v.push(4);
}

fn main() {
    let msg1 = "cat".to_string();
    // Creates a full copy
    let msg2 = msg1.clone();

    // Borrow
    // - creates reference
    // - doesn't transfer ownership
    // - can create one mutable reference
    // - or any number of immutable references
    // let msg3 = &msg1;
    print_msg(&msg1);

    let mut v = vec![1, 2, 3];
    add_to_vec(&mut v);
    println!("{:?}", v);
}
