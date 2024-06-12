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

fn main() {
    let cat = Cat {
        name: "Luna".to_string(),
    };
    let dog = Dog {
        name: "Kabosu".to_string(),
    };

    cat.chomp();
    dog.chomp();
}
