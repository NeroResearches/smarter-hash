use std::hash::Hasher;

pub trait SymmetricCombine {
    /// Combine two elements symmetrically.
    ///
    /// # Invariant
    /// for any x and y: `x.symmetric_combine(y) = y.symmetric_combine(x)`.
    fn symmetric_combine(self, rhs: Self) -> Self;
}
macro_rules! add_comb {
    ($($tps:ty)*) => {$(
        impl SymmetricCombine for $tps {
            #[inline]
            fn symmetric_combine(self, rhs: Self) -> Self {
                self.wrapping_add(rhs)
            }
        }
    )*};
}

add_comb!(u8 u16 u32 u64 u128 usize);
add_comb!(i8 i16 i32 i64 i128 isize);

pub trait StableHasher: Default {
    type Output: SymmetricCombine;

    fn write_bytes(&mut self, bytes: &[u8]);
    fn merge_from_output(&mut self, finished: Self::Output);

    fn finish(self) -> Self::Output;

    // Write methods.

    fn write_u8(&mut self, u: u8) {
        self.write_bytes(&u.to_le_bytes());
    }
    fn write_u16(&mut self, u: u16) {
        self.write_bytes(&u.to_le_bytes());
    }
    fn write_u32(&mut self, u: u32) {
        self.write_bytes(&u.to_le_bytes());
    }
    fn write_u64(&mut self, u: u64) {
        self.write_bytes(&u.to_le_bytes());
    }
    fn write_u128(&mut self, u: u128) {
        self.write_bytes(&u.to_le_bytes());
    }
    fn write_usize(&mut self, u: usize) {
        self.write_bytes(&u.to_le_bytes());
    }
}

impl<H: Hasher + Default> StableHasher for H {
    type Output = u64;

    fn write_bytes(&mut self, bytes: &[u8]) {
        <Self as Hasher>::write(self, bytes);
    }

    fn merge_from_output(&mut self, finished: Self::Output) {
        self.write_u64(finished);
    }

    fn finish(self) -> Self::Output {
        <Self as Hasher>::finish(&self)
    }

    fn write_u8(&mut self, u: u8) {
        <Self as Hasher>::write_u8(self, u);
    }

    fn write_u16(&mut self, u: u16) {
        <Self as Hasher>::write_u16(self, u);
    }

    fn write_u32(&mut self, u: u32) {
        <Self as Hasher>::write_u32(self, u);
    }

    fn write_u64(&mut self, u: u64) {
        <Self as Hasher>::write_u64(self, u);
    }

    fn write_u128(&mut self, u: u128) {
        <Self as Hasher>::write_u128(self, u);
    }

    fn write_usize(&mut self, u: usize) {
        <Self as Hasher>::write_usize(self, u);
    }
}
