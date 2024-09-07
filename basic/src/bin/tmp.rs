#![allow(unused)]

use std::collections::HashMap;
use std::path::{Path, PathBuf};

fn main() {
    // let file_path = Path::new(&args.base_path).join("src").join(&args.code_id);
    // let path = PathBuf::from(".");
    let path = Path::new(".");
    let is_dir = path.is_dir();
    let is_file = path.is_file();
    println!("is_dir: {is_dir}");
    println!("is_file : {is_file}");
}
