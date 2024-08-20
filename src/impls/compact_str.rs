use compact_str::CompactString;

use crate::StableHash;

impl StableHash for CompactString {
    #[inline]
    fn stable_hash<H>(&self, hasher: &mut H)
    where
        H: crate::StableHasher,
    {
        self.as_str().stable_hash(hasher);
    }
}
