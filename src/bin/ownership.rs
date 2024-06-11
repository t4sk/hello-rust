// - each value has only one owner at a time
// - value is dropped when the owner goes out of scope

fn print_hello(s: String) {
    println!("{}", s);
}

fn main() {
    // Only one owner
    // This will not compile
    // let s1 = "Hello".to_string();
    // let s2 = s1;
    // println!("{}", s1);

    // Value is dropped when the owner goes out of scope
    // This will not compile
    // let s = "Hello".to_string();
    // print_hello(s);
    // println!("Again: {}", s);
}
