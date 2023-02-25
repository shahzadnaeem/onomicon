use crate::hashers::hasher;

#[derive(Debug)]
pub struct Simple {
    pub key: String,
    pub value: String,
    pub hash: u64,
}

impl Simple {
    pub fn new(key: &str, value: &str) -> Self {
        Simple {
            key: key.to_string(),
            value: value.to_string(),
            hash: hasher::hasher_u64(key),
        }
    }
}
