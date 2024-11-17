// Each value has only one owner at a time
// Value is dropped when the owner goes out of scope

// scope - range within a program where an item (variable and function) is valid

fn take(s: String) {
    println!("{}", s);
}

fn give() -> String {
    String::from("cat")
}

fn take_and_give(s: String) -> String {
    println!("take {}", s);
    s
}

fn main() {
    // Only one owner
    // This will not compile
    // let s1 = "Hello".to_string();
    // let s2 = s1;
    // println!("{}", s1);

    // Value is dropped when the owner goes out of scope
    {
        let s = "hello";
        println!("{}", s);
    } // this scope is now over, and s is no longer valid

    // This will not compile
    // println!("{}", s);

    let s = "Hello".to_string();
    take(s);
    // This will not compile
    // println!("Again: {}", s);

    let s = give();
    println!("{}", s);

    let mut s = "cat".to_string();
    s = take_and_give(s);
    println!("{}", s);

    // Move - transfer ownership
    // No transfer of ownership
    let x = 5;
    let y = x; // value of x is copied into y

    // Transfer of ownership
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}", s1);
    println!("{}", s2);

    // String and str
    // string literal is hardcoded into code
}
