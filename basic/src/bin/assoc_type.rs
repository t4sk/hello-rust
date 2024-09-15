#![allow(unused)]

// associated type are type placeholder used inside a trait
// implementation of the trait will specify the concrete type
// diff between generic?
// when to use associated type

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct List {}

impl Iterator for List {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(0)
    }
}

// More than 1 implementation -> this will not compile
// impl Iterator for List {
//     type Item = i32;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         Some(0)
//     }
// }

// generic and associated type
// generic - can be implemented for a type multiple times
// associated type - one impl for a type
struct GenericList<T> {
    val: T,
}

pub trait GenericIterator<T> {
    fn next(&mut self) -> Option<T>;
}

impl GenericIterator<u32> for GenericList<u32> {
    fn next(&mut self) -> Option<u32> {
        Some(0)
    }
}

impl GenericIterator<i32> for GenericList<i32> {
    fn next(&mut self) -> Option<i32> {
        Some(0)
    }
}

fn main() {
    let mut list = List {};
    while let Some(i) = list.next() {
        continue;
    }

    let mut list: GenericList<u32> = GenericList { val: 0 };
    while let Some(u) = list.next() {
        continue;
    }

    let mut list: GenericList<i32> = GenericList { val: 0 };
    while let Some(u) = list.next() {
        continue;
    }
}
