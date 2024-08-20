use crate::{StableHash, StableHasher, SymmetricCombine as _};

#[inline]
pub fn compute_hash<I, H>(item: I) -> H::Output
where
    H: StableHasher,
    I: StableHash,
{
    let mut current = H::default();
    item.stable_hash(&mut current);
    current.finish()
}

#[cfg(test)]
#[track_caller]
pub fn assert_ne<I>(lhs: I, rhs: impl Iterator<Item = I::Item>)
where
    I: Iterator<Item: std::fmt::Debug + PartialEq>,
{
    let lhs: Vec<_> = lhs.collect();
    let rhs: Vec<_> = rhs.collect();

    assert_ne!(lhs, rhs);
}

#[inline]
pub fn write_seq<H, I>(iter: I, to: &mut H)
where
    I: Iterator<Item: StableHash>,
    H: StableHasher,
{
    iter.for_each(|item| item.stable_hash(to));
}

#[inline]
pub fn write_seq_symm<H, I>(mut iter: I, to: &mut H)
where
    I: Iterator<Item: StableHash>,
    H: StableHasher,
{
    let Some(accum) = iter.next().map(compute_hash::<_, H>) else {
        return;
    };
    to.merge_from_output(fold_seq_symm::<_, H>(accum, iter));
}

pub fn fold_seq_symm<I, H>(accum: H::Output, iter: I) -> H::Output
where
    I: Iterator<Item: StableHash>,
    H: StableHasher,
{
    iter.fold(accum, |acc, item| {
        acc.symmetric_combine(compute_hash::<_, H>(item))
    })
}
