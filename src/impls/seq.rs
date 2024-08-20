use std::collections::{HashMap, HashSet};

use crate::{utils::write_seq_symm, StableHash};

impl<V, S> StableHash for HashSet<V, S>
where
    V: StableHash,
{
    #[inline]
    fn stable_hash<H>(&self, hasher: &mut H)
    where
        H: crate::StableHasher,
    {
        write_seq_symm(self.iter(), hasher);
    }
}

impl<K, V, S> StableHash for HashMap<K, V, S>
where
    K: StableHash,
    V: StableHash,
{
    #[inline]
    fn stable_hash<H>(&self, hasher: &mut H)
    where
        H: crate::StableHasher,
    {
        write_seq_symm(self.iter(), hasher);
    }
}

impl<T> StableHash for Vec<T>
where
    T: StableHash,
{
    #[inline]
    fn stable_hash<H>(&self, hasher: &mut H)
    where
        H: crate::StableHasher,
    {
        write_seq_symm(self.iter(), hasher);
    }
}

#[cfg(test)]
mod tests {
    use std::{fmt, hash::DefaultHasher};

    use crate::utils::compute_hash;

    #[track_caller]
    fn assert_ne<I>(lhs: I, rhs: impl Iterator<Item = I::Item>)
    where
        I: Iterator<Item: fmt::Debug + PartialEq>,
    {
        let one: Vec<_> = lhs.collect();
        let two: Vec<_> = rhs.collect();

        assert_ne!(one, two);
    }

    #[test]
    fn vec_order_does_not_matter() {
        let one = vec![1, 2, 3];
        let two = vec![3, 2, 1];

        assert_ne(one.iter(), two.iter());

        assert_eq!(
            compute_hash::<_, DefaultHasher>(one),
            compute_hash::<_, DefaultHasher>(two)
        );
    }
}
