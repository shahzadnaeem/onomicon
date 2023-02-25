// Naive implementation...

pub trait Hash64 {
    fn hash(key: &str) -> u64;
}

pub struct FixedHash;

impl Hash64 for FixedHash {
    fn hash(key: &str) -> u64 {
        key.len() as u64 + 10000u64
    }
}

pub fn hash(key: &str) -> u64 {
    FixedHash::hash(key)
}

// Flip this around

pub trait HashU64 {
    fn calc_hash_str(key: &str) -> u64;
    fn calc_hash_buf(buf: &[u8]) -> u64;
}

pub struct HashTag;

impl HashU64 for HashTag {
    fn calc_hash_str(key: &str) -> u64 {
        key.chars()
            .enumerate()
            .map(|(i, c)| ((i % 4) as u64) * (c as u64))
            .sum()
    }

    fn calc_hash_buf(key: &[u8]) -> u64 {
        key.iter()
            .enumerate()
            .map(|(i, c)| ((i % 4) as u64) * (*c as u64))
            .sum()
    }
}

pub fn hasher_u64(key: &str) -> u64 {
    HashTag::calc_hash_str(key)
}
