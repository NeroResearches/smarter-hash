use std::{
    collections::{BTreeMap, HashMap, HashSet},
    hash,
};

use crate::{traits::StableHash, utils::hash_sum};

impl<V: StableHash, S> StableHash for HashSet<V, S> {
    fn stable_hash<H>(&self, hasher: &mut H)
    where
        H: hash::Hasher + Default,
    {
        hash_sum(
            self.iter(),
            |item, hasher| {
                item.stable_hash(hasher);
            },
            hasher,
        );
    }
}

impl<K: StableHash, V: StableHash, S> StableHash for HashMap<K, V, S> {
    fn stable_hash<H>(&self, hasher: &mut H)
    where
        H: hash::Hasher + Default,
    {
        hash_sum(
            self.iter(),
            |(k, v), hasher| {
                k.stable_hash(hasher);
                v.stable_hash(hasher);
            },
            hasher,
        );
    }
}

impl<K, V> StableHash for BTreeMap<K, V>
where
    K: hash::Hash + StableHash,
    V: hash::Hash + StableHash,
{
    fn stable_hash<H>(&self, hasher: &mut H)
    where
        H: hash::Hasher + Default,
    {
        hash_sum(
            self.iter(),
            |(k, v), hasher| {
                k.stable_hash(hasher);
                v.stable_hash(hasher);
            },
            hasher,
        );
    }
}
