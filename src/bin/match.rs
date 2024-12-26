#![allow(unused)]

enum Animal {
    Cat,
    Dog,
    Duck,
    Mouse,
}

fn main() {
    // Match
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other"),
    }

    // Multiple
    match x {
        1 | 2 | 3 => println!("1 or 2 or 3"),
        _ => println!("other"),
    }

    // Range
    match x {
        1..=10 => println!("between 1 to 10"),
        _ => println!("other"),
    }

    // @
    match x {
        i @ 1..=10 => println!("@ {i}"),
        _ => println!("other"),
    }

    // Assigning value from match
    let animal = Animal::Cat;
    let animal_sound = match animal {
        Animal::Cat => "meow",
        Animal::Dog => "woof",
        Animal::Duck => "quack",
        _ => "?",
    };
    println!("{}", animal_sound);

    // Option
    let x: Option<u32> = Some(3);
    match x {
        Some(x) => println!("{x}"),
        None => println!("none"),
    }

    // Result
    let res: Result<u32, String> = Ok(1);
    match res {
        Ok(i) => println!("ok {i}"),
        // Ignore error message
        Err(_) => println!("err"),
    }
}
