#![allow(unused)]

// A trait defines the functionality a particular type has and can share with
// other types

// data -> struct
// behavior -> trait

struct Cat {
    name: String,
}

struct Dog {
    name: String,
}

trait Eat {
    fn chomp(&self);
    // default implementation
    //fn chomp(&self) {
    //    println!("chomp!");
    //}
}

impl Eat for Cat {
    fn chomp(&self) {
        println!("{} chomps fishes", self.name);
    }
}

impl Eat for Dog {
    fn chomp(&self) {
        println!("{} chomps bones", self.name);
    }
}

trait Emoji {
    fn print_emoji(&self);
}

impl Emoji for Cat {
    fn print_emoji(&self) {
        println!("🐱 {}", self.name);
    }
}

impl Emoji for Dog {
    fn print_emoji(&self) {
        println!("🐶 {}", self.name);
    }
}

fn main() {
    let cat = Cat {
        name: "Luna".to_string(),
    };

    let dog = Dog {
        name: "Kabosu".to_string(),
    };

    cat.chomp();
    dog.chomp();

    cat.print_emoji();
    dog.print_emoji();
}
