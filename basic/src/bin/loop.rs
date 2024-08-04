fn main() {
    // loop
    let mut i: u32 = 0;
    loop {
        println!("loop {}", i);
        i += 1;

        if i > 5 {
            break;
        }
    }

    // while loop
    let mut i: u32 = 0;
    while i <= 5 {
        println!("while loop {}", i);
        i += 1;
    }

    // for loop
    for i in 0..5 {
        println!("for loop {}", i);
    }

    let xs = [1, 2, 3];
    for x in xs.iter() {
        println!("for loop iter {}", x);
    }

    // return value from loop
    let mut i: u32 = 0;
    let v = loop {
        i += 1;
        if i > 3 {
            break "i > 3";
        }
    };
    println!("return value from loop {}", v);
}
