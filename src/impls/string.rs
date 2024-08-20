use crate::StableHash;

impl StableHash for str {
    #[inline]
    fn stable_hash<H>(&self, hasher: &mut H)
    where
        H: crate::StableHasher,
    {
        hasher.write_bytes(self.as_bytes());
    }
}

impl StableHash for String {
    #[inline]
    fn stable_hash<H>(&self, hasher: &mut H)
    where
        H: crate::StableHasher,
    {
        self.as_str().stable_hash(hasher);
    }
}
