use std::hash;

use crate::{traits::StableHash, utils::hash_sum};

impl<T: StableHash> StableHash for Vec<T> {
    fn stable_hash<H>(&self, hasher: &mut H)
    where
        H: hash::Hasher + Default,
    {
        hash_sum(self.iter(), |item, hasher| item.stable_hash(hasher), hasher);
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::compute_hash;

    #[test]
    fn order_does_not_matter() {
        let one = vec![1, 2, 3];
        let two = vec![3, 2, 1];

        assert_eq!(compute_hash(one), compute_hash(two));
    }
}
