#![allow(unused)]

fn main() {
    // Array - fixed size, known at compile time
    let arr: [u32; 3] = [1, 2, 3];
    println!("arr[0] = {}", arr[0]);

    // Tuples - fixed size, known at compile time
    let tup: (bool, char, u32) = (true, 'c', 3);
    println!("tup 0 = {}", tup.0);
    println!("tup 1 = {}", tup.1);
    println!("tup 2 = {}", tup.2);

    let empty_tup = ();

    // slice
    let arr: [u32; 5] = [1, 2, 3, 4, 5];
    let slice = &arr[1..3];
    println!("slice {:?}", slice);

    // String and str
    // TODO: what's the difference between string and str?
    let my_string = String::from("my string");
    let my_string = "my string".to_string();
    let my_str = "my str";

    let string_slice = &my_string[1..3];
    println!("string slice {:?}", string_slice);
}
