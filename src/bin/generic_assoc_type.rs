#![allow(unused)]

// Associated type
// - placeholder type inside trait definition
// - placeholder is replaced by the implementation

// Difference with generic trait
// - generic = multiple implementation per type
// - assoc type = 1 implementation per type

// Generic trait
trait GenericIterator<T> {
    fn get_next(&mut self) -> Option<T>;
}

struct Counter {
    pub count: u32,
}

impl GenericIterator<u32> for Counter {
    fn get_next(&mut self) -> Option<u32> {
        self.count += 1;
        Some(self.count)
    }
}

impl GenericIterator<bool> for Counter {
    fn get_next(&mut self) -> Option<bool> {
        Some(true)
    }
}

// Associated type
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        Some(self.count)
    }
}

// This will not compile - only one implementation for associated type
/*
impl Iterator for Counter {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        Some(true)
    }
}
*/

fn main() {
    let mut counter = Counter { count: 0 };

    let v = counter.next();
    println!("{:?}", v);

    let v = counter.next();
    println!("{:?}", v);

    let v = counter.next();
    println!("{:?}", v);
}
