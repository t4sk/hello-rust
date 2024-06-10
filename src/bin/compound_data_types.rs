fn main() {
    // Array - fixed size, known at compile time
    let arr: [u32; 3] = [1, 2, 3];
    println!("arr[0] = {}", arr[0]);

    // Tuples - fixed size, known at compile time
    let tup: (bool, char, u32) = (true, 'c', 3);
    println!("tup 0 = {}", tup.0);
    println!("tup 1 = {}", tup.1);
    println!("tup 2 = {}", tup.2);

    let empty_tup = ();
}
