#![allow(unused)]

fn zeros(nums: &[u32]) -> &[u32] {
    for (i, &num) in nums.iter().enumerate() {
        if num != 0 {
            return &nums[0..i];
        }
    }

    return &nums[..];
}

fn ones(nums: &[u32]) -> &[u32] {
    for (i, &num) in nums.iter().enumerate() {
        if num == 0 {
            continue;
        }
        return &nums[i..nums.len()];
    }

    return &nums[0..0];
}

fn main() {
    // slice is a reference to an collection so doesn't have ownership
    let s = String::from("hello world");

    // &s[..5]
    let hello = &s[0..5];
    // &s[6..]
    let world = &s[6..11];

    let nums = [0, 0, 0, 0, 1, 1];
    let b: &[u32] = &nums[..];

    let z = zeros(&nums);
    println!("{:?}", z);

    let o = ones(&nums);
    println!("{:?}", o);
}
