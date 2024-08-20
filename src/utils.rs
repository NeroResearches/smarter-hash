use std::hash::{self, Hasher as _};

use crate::StableHash;

// for tests
#[allow(dead_code)]
pub fn compute_hash<T>(value: T) -> u64
where
    T: StableHash,
{
    let mut hasher = hash::DefaultHasher::new();
    value.stable_hash(&mut hasher);
    hasher.finish()
}

pub fn hash_sum<I, H, F>(iter: I, mut f: F, hasher: &mut H)
where
    I: Iterator,
    H: hash::Hasher + Default,
    F: FnMut(I::Item, &mut H),
{
    let mut result = 0_u64;
    for item in iter {
        let mut current = H::default();
        f(item, &mut current);
        result = result.wrapping_add(current.finish());
    }
    hasher.write_u64(result);
}
