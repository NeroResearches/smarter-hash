use crate::StableHash;

impl<O: StableHash, E: StableHash> StableHash for Result<O, E> {
    #[inline]
    fn stable_hash<H>(&self, hasher: &mut H)
    where
        H: crate::StableHasher,
    {
        match self {
            Self::Ok(o) => {
                true.stable_hash(hasher);
                o.stable_hash(hasher);
            }
            Self::Err(e) => {
                false.stable_hash(hasher);
                e.stable_hash(hasher);
            }
        }
    }
}

impl<T: StableHash> StableHash for Option<T> {
    #[inline]
    fn stable_hash<H>(&self, hasher: &mut H)
    where
        H: crate::StableHasher,
    {
        if let Some(x) = self {
            x.stable_hash(hasher);
        }
    }
}
