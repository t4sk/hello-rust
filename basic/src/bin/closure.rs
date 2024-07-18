// Closures are functions that can capture the enclosing environment
// - can save in a variable
// - can pass as function argument
fn main() {
    let outer_var = 42;

    let closure_annotated = |i: i32| -> i32 { i + outer_var };
    let closure_inferred = |i| i + outer_var;

    // Call the closures.
    println!("closure_annotated: {}", closure_annotated(1));
    println!("closure_inferred: {}", closure_inferred(1));

    // Using move before vertical pipes forces closure to take ownership of captured variables:
    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    let id = |x| x;
    let s = id("hello".to_string());
    // infered type is locked to String -> String
    //let n = id(5);
}
