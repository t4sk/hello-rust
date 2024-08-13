#![allow(unused)]

fn print_msg(msg: &String) {
    println!("Message is {}", msg);
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

fn print_vec(v: Vec<i32>) { // Takes ownership of v
    println!("{:?}", v);
} // v is dropped

fn print_vec_return_ownership(v: Vec<i32>) -> Vec<i32> { // Takes ownership of v
    println!("{:?}", v);
    v
}

fn print_vec_borrow(v: &Vec<i32>) {
    println!("{:?}", v);
}

fn add_to_vec(v: &mut Vec<i32>) {
    v.push(4);
}

fn main() {
    // Borrow example
    let v = vec![1, 2, 3];
    print_vec(v);
    // This will not compile
    // println!("{:?}", v);

    // Return ownership
    let v = vec![1, 2, 3];
    let v = print_vec_return_ownership(v);
    println!("{:?}", v);

    // Borrow
    let v = vec![1, 2, 3];
    print_vec_borrow(&v) ;
    println!("{:?}", v);

    // Mutable borrow example
    let mut v = vec![1, 2, 3];
    add_to_vec(&mut v);
    println!("{:?}", v);

    let s = "cat".to_string();
    // Creates a full copy
    let s1 = s.clone();

    // Borrow
    // - creates reference
    // - doesn't transfer ownership
    // - can create one mutable reference
    // - or any number of immutable references

    // One mutable reference (at a time) //
    let mut s = String::from("cat");
    let s1 = &mut s;
    // This will not compile
    // let s2 = &mut s;
    s1.push_str(" meow");

    // Multiple mutable reference (only one at a time) //
    {
        let s1 = &mut s;
        s1.push_str(" meow");
        println!("{}", s1);
    }

    let s2 = &mut s;
    s2.push_str("!");
    println!("{}", s2);

    // Multiple immutable reference
    let mut s = String::from("dog");
    let s1 = &s;
    let s2 = &s;
    println!("s1 {}", s1);
    println!("s2 {}", s2);
}

// Dangling reference
// does not compile

// fn main() {
//     let reference_to_nothing = dangle();
// }
//
// fn dangle() -> &String {
//     let s = String::from("hello");
//
//     &s
// }
