#![allow(unused)]

#[derive(Debug)]
enum MathError {
    DivByZero,
    Other,
}

fn main() {
    // Panic - crash the program
    panic!("crash");

    // This will also panic - index out of bound
    let v = [1, 2, 3];
    v[99];

    // Option
    let x = v.get(99);
    // Error handling with Option
    match x {
        Some(i) => println!("{i}"),
        None => println!("none"),
    }

    let x = 1;
    let y = 0;
    // This will panic - division by 0
    let q = x / y;

    // Result
    let q = if y != 0 {
        Ok(x / y)
    } else {
        Err(MathError::DivByZero)
    };
    // Error handling with Result
    match q {
        Ok(r) => println!("{x} / {y} = {r}"),
        Err(err) => println!("{x} / {y} {:?}", err),
    }
}
