use std::hash;

pub(crate) mod macros;

use macros::{transparent, tup};

pub trait StableHash {
    fn stable_hash<H>(&self, hasher: &mut H)
    where
        H: hash::Hasher + Default;

    fn stable_hash_slice<H>(data: &[Self], hasher: &mut H)
    where
        Self: Sized,
        H: hash::Hasher + Default,
    {
        for item in data {
            item.stable_hash(hasher);
        }
    }
}

impl<'a, T: StableHash> StableHash for &'a T {
    fn stable_hash<H>(&self, hasher: &mut H)
    where
        H: hash::Hasher + Default,
    {
        <T as StableHash>::stable_hash(self, hasher)
    }
}
impl<'a, T: StableHash + ?Sized> StableHash for &'a mut T {
    fn stable_hash<H>(&self, hasher: &mut H)
    where
        H: hash::Hasher + Default,
    {
        <T as StableHash>::stable_hash(&*self, hasher)
    }
}

transparent!(u8, u16, u32, u64, u128, usize);
transparent!(i8, i16, i32, i64, i128, isize);
transparent!(bool);
transparent!(@!Sized str, String);

tup!(T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10);
tup!(T0 T1 T2 T3 T4 T5 T6 T7 T8 T9);
tup!(T0 T1 T2 T3 T4 T5 T6 T7 T8);
tup!(T0 T1 T2 T3 T4 T5 T6 T7);
tup!(T0 T1 T2 T3 T4 T5 T6);
tup!(T0 T1 T2 T3 T4 T5);
tup!(T0 T1 T2 T3 T4);
tup!(T0 T1 T2 T3);
tup!(T0 T1 T2);
tup!(T0 T1);
