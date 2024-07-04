fn main() {
    // if / else
    let x = 1;

    if x == 1 {
        println!("x = 1");
    } else if x == 2 {
        println!("x = 2");
    } else {
        println!("x >= 3");
    }

    // Assigning value from if / else
    let x = 1;
    let y = if x % 2 == 0 { "even" } else { "odd" };
    // Note - semicolon above
    println!("{} is {}", x, y);

    // match
    let x = 1;

    match x {
        1 => println!("1"),
        2 => println!("2"),
        3 => println!("3"),
        _ => println!("other"),
    }

    // Assigning value from match
    let x = 4;
    let animal = match x {
        1 => "dog",
        2 => "cat",
        3 => "mouse",
        _ => "animal",
    };
    println!("{}", animal);
}
