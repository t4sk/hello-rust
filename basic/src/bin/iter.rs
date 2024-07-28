// iterator pattern allows you to perform some task on a sequence of items
// iterator is responsible for the logic of iterating over each item
// iterators are lazy

fn main() {
    let v = vec![1, 2, 3];

    for v in v.iter() {
        println!("v {}", v);
    }

    // Need to make iterator mutable
    let mut v1_iter = v.iter();
    println!("{:?}", v1_iter.next());
    println!("{:?}", v1_iter.next());
    println!("{:?}", v1_iter.next());
    println!("{:?}", v1_iter.next());

    // some useful methods
    let s: i32 = v.iter().sum();
    println!("sum {}", s);

    let v2: Vec<i32> = v.iter().map(|x| x + 1).collect();
    println!("v2 {:?}", v2);
}
