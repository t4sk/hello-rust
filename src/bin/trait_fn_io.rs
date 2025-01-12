#![allow(unused)]

trait Animal {
    fn speak(&self) -> String;
}

struct Cat;
struct Dog;

impl Animal for Cat {
    fn speak(&self) -> String {
        "meow".to_string()
    }
}

impl Animal for Dog {
    fn speak(&self) -> String {
        "woof".to_string()
    }
}

// Trait as input - animal must be a type known at compile time
fn greet(animal: &impl Animal) {
    println!("{} world!", animal.speak());
}

// Trait as input - animal is a type not known at compile time
fn greet_dyn(animal: &dyn Animal) {
    println!("{} world!", animal.speak());
}

// Trait as output when return type is a single type known at compile time
fn return_concrete_type() -> impl Animal {
    Dog
}

// Trait as ouput when return type is unknown until run time
// Need to wrap in a Box
fn rand_animal(rand: f32) -> Box<dyn Animal> {
    if rand <= 0.5 {
        Box::new(Cat)
    } else {
        Box::new(Dog)
    }
}

fn main() {
    let cat = Cat;
    let dog = Dog;

    greet(&cat);
    greet(&dog);

    let input = "dog";
    let animal: &dyn Animal = match input {
        "dog" => &Dog,
        _ => &Cat,
    };

    greet_dyn(animal);

    let dog = return_concrete_type();
    println!("{}", dog.speak());

    let animal = rand_animal(0.4);
    println!("{}", animal.speak());
}
