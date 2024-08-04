#![allow(unused)]
fn main() {
    let x = 1;
    let v = if x < 2 { "a" } else { "b" };

    let v = {
        let a = 1;
        let b = 2;
        a + b
    };
}
