#![allow(unused)]

struct Val<T> {
    val: T
}

impl<T> Val<T> {
    fn get_value(&self) -> &T {
        &self.val
    }
}

fn main() {
    let x = Val {val: 123i32};
    println!("{}", x.get_value());

}
