#![allow(unused)]

mod foo {
    pub fn print() {
        println!("foo");
    }
}

mod my {
    pub fn print() {
        println!("my");
    }

    // Private - cannot be called by main
    fn f() {
        println!("private");
    }

    // Nest modules
    pub mod a {
        pub fn print() {
            println!("a");
        }

        // Public struct
        pub struct S {
            pub name: String,
            // Private field
            id: u32,
        }

        pub fn build(name: String) -> S {
            S { name, id: 1 }
        }
    }

    // Private module
    // Cannot be called outside of this module
    mod b {
        pub fn print() {
            println!("b");
        }
    }

    fn g() {
        b::print();
    }

    // Go one level up in the module tree
    use super::foo;

    fn call_foo_print() {
        foo::print();
    }
}

use my::a::print as a_print;

fn main() {
    my::print();
    my::a::print();
    a_print();
    let s = my::a::build("rust".to_string());
}
