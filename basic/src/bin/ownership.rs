// Each value has only one owner at a time
// Value is dropped when the owner goes out of scope

// scope - range within a program where an item (variable and function) is valid

fn take(s: String) {
    println!("{}", s);
}

fn main() {
    {
        let s = "hello";
        println!("{}", s);
    } // this scope is now over, and s is no longer valid

    // String and str
    // string literal is hardcoded into code

    // Only one owner
    // This will not compile
    // let s1 = "Hello".to_string();
    // let s2 = s1;
    // println!("{}", s1);

    // Value is dropped when the owner goes out of scope
    // This will not compile
    // let s = "Hello".to_string();
    // take(s);
    // println!("Again: {}", s);

    // Move - transfer ownership
    // No transfer of ownership
    let x = 5;
    let y = x; // value of x is copied into y

    // Transfer of ownership
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}", s1);
    println!("{}", s2);
}
