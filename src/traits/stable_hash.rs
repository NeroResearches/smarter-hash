use super::stable_hasher::StableHasher;

pub trait StableHash {
    fn stable_hash<H>(&self, hasher: &mut H)
    where
        H: StableHasher;
}
