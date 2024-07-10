use std::collections::HashMap;

fn main() {
    // Vec<T>
    // HashMap<K, V>
    // HashSet<T>
    // VecDeque<T>
    let v = vec![1, 2, 3];

    let v = vec![1u8, 2, 3, 4];
    println!("vec = {:?}", v);

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    println!("vec = {:?}", v);

    let x = v[1];
    println!("{}", x);

    let x = v.get(1);
    match x {
        Some(val) => println!("val {:?}", val),
        None => println!("value doesn't exist"),
    }

    // Slice
    let v_slice = &v[1..3];
    println!("vec slice = {:?}", v_slice);

    let mut map = HashMap::new();
    map.insert("a".to_string(), 1u8);
    map.insert("b".to_string(), 2u8);

    let val = map.get("a");
    println!("val = {:?}", val);
}
