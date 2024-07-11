fn div(x: u32, y: u32) -> Result<u32, String> {
    if y == 0 {
        return Err("Division by 0".to_string());
    }
    Ok(x / y)
}

fn main() {
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
}
