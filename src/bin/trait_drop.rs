#![allow(unused)]

struct A {
    name: String,
}

impl Drop for A {
    fn drop(&mut self) {
        println!("Dropping {}", self.name);
    }
}

fn main() {
    let a = A {
        name: "a".to_string(),
    };
    let b = A {
        name: "b".to_string(),
    };
    let c = A {
        name: "c".to_string(),
    };

    std::mem::drop(b);

    {
        c;
    }
}
