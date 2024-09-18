#![allow(unused)]

fn main() {
    // map
    let a: Vec<u32> = vec![1, 2, 3, 4, 5, 6];
    // returns iterator without collect()
    let b: Vec<u32> = a.iter().map(|v| 2 * v).collect();
    let c: Vec<u32> = a.into_iter().filter(|v| *v % 2 == 0).collect();
    println!("{:?}", b);
    println!("{:?}", c);

    // filter_map
    let a: Vec<u32> = vec![1, 2, 3, 4, 5, 6];
    let d: Vec<u32> = a
        .iter()
        .filter_map(|v| if *v <= 3 { None } else { Some(v * v) })
        .collect();
    println!("{:?}", d);

    // fold
    let a: Vec<u32> = vec![1, 2, 3, 4, 5, 6];
    let sum = a.iter().fold(0, |acc, x| acc + x);
    println!("sum {}", sum);
}
