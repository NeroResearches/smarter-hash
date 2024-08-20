use std::{hash, rc::Rc, sync::Arc};

use crate::StableHash;

impl<T: StableHash + ?Sized> StableHash for Rc<T> {
    fn stable_hash<H>(&self, hasher: &mut H)
    where
        H: hash::Hasher + Default,
    {
        <T as StableHash>::stable_hash(&*self, hasher)
    }
}

impl<T: StableHash + ?Sized> StableHash for Arc<T> {
    fn stable_hash<H>(&self, hasher: &mut H)
    where
        H: hash::Hasher + Default,
    {
        <T as StableHash>::stable_hash(&*self, hasher)
    }
}

impl<T: StableHash + ?Sized> StableHash for Box<T> {
    fn stable_hash<H>(&self, hasher: &mut H)
    where
        H: hash::Hasher + Default,
    {
        <T as StableHash>::stable_hash(&*self, hasher)
    }
}
