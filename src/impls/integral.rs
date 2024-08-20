use crate::StableHash;

macro_rules! impl_uint {
    ($($ty:ident:$write_method:ident $([$cast:ty])?)*) => {
        $(
            impl StableHash for $ty {
                #[inline]
                fn stable_hash<H>(&self, hasher: &mut H)
                where
                    H: crate::StableHasher,
                {
                    hasher.$write_method(*self $(as $cast)?);
                }
            }
        )*
    };
}

impl_uint! {
    u8:write_u8
    u16:write_u16
    u32:write_u32
    u64:write_u64
    u128:write_u128

    i8:write_u8[u8]
    i16:write_u16[u16]
    i32:write_u32[u32]
    i64:write_u64[u64]
    i128:write_u128[u128]

    bool:write_u8[u8]
}
