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

struct ArrayIter<T> {
    array: [T; 5],
    i: usize,
}

impl GenericIterator<u32> for ArrayIter<u32> {
    fn get_next(&mut self) -> Option<u32> {
        match self.array.get(self.i) {
            Some(v) => {
                self.i += 1;
                Some(*v)
            }
            _ => None,
        }
    }
}

impl GenericIterator<bool> for ArrayIter<u32> {
    fn get_next(&mut self) -> Option<bool> {
        Some(true)
    }
}

// Associated type
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

impl Iterator for ArrayIter<u32> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.array.get(self.i) {
            Some(v) => {
                self.i += 1;
                Some(*v)
            }
            _ => None,
        }
    }
}

// This will not compile - only one implementation for associated type
/*
impl Iterator for ArrayIter<u32> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        Some(true)
    }
}
*/

fn main() {
    let mut arr_iter: ArrayIter<u32> = ArrayIter {
        array: [1, 2, 3, 4, 5],
        i: 0,
    };

    while let Some(v) = arr_iter.next() {
        println!("{:?}", v);
    }
}
