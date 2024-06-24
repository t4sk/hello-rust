use std::collections::HashMap;

fn main() {
    // Vec<T>
    // HashMap<K, V>
    // HashSet<T>
    // VecDeque<T>
    let v = vec![1u8, 2, 3, 4];
    println!("vec = {:?}", v);

    let v_slice = &v[1..3];
    println!("vec slice = {:?}", v_slice);

    let mut v = vec![1u8, 2, 3, 4];
    v.push(5);
    println!("vec = {:?}", v);

    let mut map = HashMap::new();
    map.insert("a".to_string(), 1u8);
    map.insert("b".to_string(), 2u8);

    let val = map.get("a");
    println!("val = {:?}", val);
}
