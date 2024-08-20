use indexmap::{IndexMap, IndexSet};

use crate::{write_seq_symm, StableHash};

impl<K, V, S> StableHash for IndexMap<K, V, S>
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

impl<T, S> StableHash for IndexSet<T, S>
where
    T: StableHash,
{
    fn stable_hash<H>(&self, hasher: &mut H)
    where
        H: crate::StableHasher,
    {
        write_seq_symm(self.iter(), hasher);
    }
}

#[cfg(test)]
mod tests {
    use std::hash::DefaultHasher;

    use indexmap::{IndexMap, IndexSet};

    use crate::{compute_hash, utils::assert_ne};

    #[test]
    fn indexset_order_does_not_matter() {
        let lhs: IndexSet<&str> = ["Hello", "World", "!"].into_iter().collect();
        let rhs: IndexSet<&str> = ["World", "!", "Hello"].into_iter().collect();

        assert_ne(lhs.iter(), rhs.iter());

        assert_eq!(
            compute_hash::<_, DefaultHasher>(lhs),
            compute_hash::<_, DefaultHasher>(rhs),
        );
    }

    #[test]
    fn indexmap_order_does_not_matter() {
        let lhs: IndexMap<&str, &str> = [("Hello", "World"), ("Lol", "Kek")].into_iter().collect();
        let rhs: IndexMap<&str, &str> = [("Lol", "Kek"), ("Hello", "World")].into_iter().collect();

        assert_ne(lhs.iter(), rhs.iter());

        assert_eq!(
            compute_hash::<_, DefaultHasher>(lhs),
            compute_hash::<_, DefaultHasher>(rhs)
        );
    }
}
