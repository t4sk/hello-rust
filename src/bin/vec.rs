fn main() {
    let arr: Vec<u32> = vec![1, 2, 3, 4, 5];
    let x: u32 = 3;

    let res = arr.iter().find(|&&i| i == x);
    println!("find {:?}", res);
}
