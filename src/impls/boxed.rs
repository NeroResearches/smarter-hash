use std::{rc::Rc, sync::Arc};

use crate::StableHash;

macro_rules! boxed {
    ($($boxed:ident)*) => {
        $(
            impl<T> StableHash for $boxed<T>
            where
                T: StableHash + ?Sized,
            {
                #[inline]
                fn stable_hash<H>(&self, hasher: &mut H)
                where
                    H: crate::StableHasher,
                {
                    <T as StableHash>::stable_hash(&*self, hasher);
                }
            }
        )*
    };
}

impl<'a, T: StableHash + ?Sized> StableHash for &'a mut T {
    #[inline]
    fn stable_hash<H>(&self, hasher: &mut H)
    where
        H: crate::StableHasher,
    {
        <T as StableHash>::stable_hash(&*self, hasher);
    }
}

impl<'a, T: StableHash + ?Sized> StableHash for &'a T {
    #[inline]
    fn stable_hash<H>(&self, hasher: &mut H)
    where
        H: crate::StableHasher,
    {
        <T as StableHash>::stable_hash(self, hasher);
    }
}

boxed!(Box Arc Rc);
