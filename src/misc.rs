use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn main() {
    let mut hasher = DefaultHasher::new();

    let num = 1234;

    num.hash(&mut hasher);

    println!("Hash of num: {} is {:x}", num, hasher.finish());
}
