use crate::{StableHash, StableHasher};

macro_rules! tup {
    ($($idents:ident)*) => {
        impl<$($idents),*> StableHash for ($($idents),*,)
        where
            $($idents: StableHash),*
        {
            #[inline]
            fn stable_hash<H>(&self, hasher: &mut H)
            where
                H: StableHasher,
            {
                #[allow(non_snake_case)]
                let ($(ref $idents),*,) = self;
                $(
                    $idents.stable_hash(hasher);
                )*
            }
        }
    };
}

tup!(T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11);
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
tup!(T0);
