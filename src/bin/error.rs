fn div(x: u32, y: u32) -> Result<u32, String> {
    if y == 0 {
        return Err("Division by 0".to_string());
    }
    Ok(x / y)
}

fn main() {
    // Unrecoverable state - halt program
    // panic!("PANIC");

    // Index out of bound error
    // let v = vec![1, 2, 3];
    // println!("{}", v[5]);

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

    // Result
    let res = div(5, 2);
    match res {
        Ok(num) => println!("5 / 2 = {}", num),
        Err(msg) => println!("Error: {}", msg),
    }

    let res = div(5, 0);
    match res {
        Ok(num) => println!("5 / 2 = {}", num),
        Err(msg) => println!("Error: {}", msg),
    }

    // TODO: Option - unwrap, expect, ?
    // TODO: Result - unwrap, expect, ?
}
