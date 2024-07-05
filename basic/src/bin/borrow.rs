fn print_msg(msg: &String) {
    println!("Message is {}", msg);
}
// Here, s goes out of scope. But because it does not have ownership of what
// it refers to, it is not dropped.

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

    // One mutable reference (at a time) - code below will not compile
    // let mut s = String::from("hello");
    // let s1 = &mut s;
    // let s2 = &mut s;
    // println!("{} {}", s1, s2);
    
    // Multiple mutable reference (only one at a time)
    let mut s = String::from("hello");
    {
        let s1 = &mut s;
        s1.push_str("world");
        println!("s1 {}", s1);
    }

    let s2 = &mut s;
    s2.push_str("!");
    println!("s2 {}", s2);

    // Multiple immutable reference
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{r3}");
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




}
