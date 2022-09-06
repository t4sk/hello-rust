use std::collections::HashMap;

fn main() {
    let mut map: HashMap<&str, u32> = HashMap::new();

    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);

    println!("get a - {:?}", map.get("a"));
    println!("get z - {:?}", map.get("z"));

    let res = map.iter().find(|(&&key, &&val)| val > 0);
    // println!("find {:?}", res);
}
