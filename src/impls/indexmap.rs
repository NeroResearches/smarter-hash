use std::hash;

use indexmap::{IndexMap, IndexSet};

use crate::{utils::hash_sum, StableHash};

impl<K: StableHash, V: StableHash, S> StableHash for IndexMap<K, V, S> {
    fn stable_hash<H>(&self, hasher: &mut H)
    where
        H: hash::Hasher + Default,
    {
        hash_sum(
            self.iter(),
            |(k, v), h| {
                k.stable_hash(h);
                v.stable_hash(h);
            },
            hasher,
        );
    }
}
impl<T: StableHash, S> StableHash for IndexSet<T, S> {
    fn stable_hash<H>(&self, hasher: &mut H)
    where
        H: hash::Hasher + Default,
    {
        hash_sum(self.iter(), |item, h| item.stable_hash(h), hasher);
    }
}

#[cfg(test)]
mod tests {
    use std::fmt;

    use indexmap::{IndexMap, IndexSet};

    use crate::utils::compute_hash;

    #[track_caller]
    fn order_not_equal<T>(lhs: impl Iterator<Item = T>, rhs: impl Iterator<Item = T>)
    where
        T: PartialEq + fmt::Debug,
    {
        let lhs: Vec<T> = lhs.collect();
        let rhs: Vec<T> = rhs.collect();

        assert_ne!(lhs, rhs);
    }

    #[test]
    fn index_set_order_does_not_matter() {
        let one: IndexSet<i32> = IndexSet::from_iter([100, 200, 300]);
        let two: IndexSet<i32> = IndexSet::from_iter([300, 200, 100]);

        order_not_equal(one.iter().copied(), two.iter().copied());

        assert_eq!(compute_hash(one), compute_hash(two));
    }

    #[test]
    fn index_map_order_does_not_matter() {
        let one: IndexMap<i32, i32> = IndexMap::from_iter([(100, 0), (200, 1), (300, 2)]);
        let two: IndexMap<i32, i32> = IndexMap::from_iter([(300, 2), (200, 1), (100, 0)]);

        order_not_equal(
            one.iter().map(|(k, v)| (*k, *v)),
            two.iter().map(|(k, v)| (*k, *v)),
        );

        assert_eq!(compute_hash(one), compute_hash(two));
    }
}
