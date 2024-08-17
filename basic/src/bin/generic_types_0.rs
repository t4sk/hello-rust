#![allow(unused)]

fn max<T: std::cmp::PartialOrd>(arr: &[T]) -> &T {
    let mut largest = &arr[0];

    for item in arr {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let nums = vec![33, 1, 22, 54, 25, 99, 10];
    let largest = max(&nums);
    println!("largest num: {}", largest);

    let chars = vec!['a', 'c', 'y', 'i', 'm'];
    let largest = max(&chars);
    println!("largest char: {}", largest);
}
