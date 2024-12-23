#![allow(unused)]

fn main() {
    // Array - fixed size, known at compile time
    let arr: [u32; 3] = [1, 2, 3];
    println!("arr[0] = {}", arr[0]);

    // Write
    let mut arr: [u32; 3] = [1, 2, 3];
    arr[1] = 9;

    // All elements = 0
    let arr: [i32; 10] = [0; 10];
    println!("{:?}", arr);

    // Slice - size not known at compile time
    let nums: [i32; 10] = [-1, 1, -2, 2, -3, 3, -4, 4, -5, 5];

    // First 3
    let s: &[i32] = &nums[0..3];
    // Optional 0 index
    let s = &nums[..3];
    println!("{:?}", s);

    // Middle 4
    let s = &nums[3..7];
    println!("{:?}", s);

    // Last 3
    let s = &nums[7..10];
    // Optional last index
    let s = &nums[7..];
    println!("{:?}", s);

    // All
    let s = &nums[..];
    println!("{:?}", s);
}
