#![allow(unused)]

fn main() {
    // Array - fixed size, known at compile time
    let arr: [u32; 3] = [1, 2, 3];
    println!("arr[0] = {}", arr[0]);

    // all elements = 0
    let arr: [i32; 10] = [0; 10];
    println!("arr {:?}", arr);

    // slice - pointer to section of an array
    let arr: [u32; 5] = [1, 2, 3, 4, 5];
    let slice = &arr[1..3];
    println!("slice {:?}", slice);
}
