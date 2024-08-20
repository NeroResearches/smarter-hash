use std::borrow::Cow;

use crate::StableHash;

impl<'a, T> StableHash for Cow<'a, T>
where
    T: Clone + StableHash + ?Sized,
{
    #[inline]
    fn stable_hash<H>(&self, hasher: &mut H)
    where
        H: crate::StableHasher,
    {
        <T as StableHash>::stable_hash(&*self, hasher);
    }
}
