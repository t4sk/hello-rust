fn main() {
    // Option<T> = Some(T) | None
    let v = vec![1, 2, 3];

    let val = v.get(1);
    match val {
        Some(x) => println!("element at 1 = {}", x),
        None => println!("index out of bound"),
    }

    let val = v.get(5);
    match val {
        Some(x) => println!("element at 5 = {}", x),
        None => println!("index out of bound"),
    }
}
