macro_rules! tup {
    ($($idents:ident)*) => {
        impl<$($idents),*> $crate::traits::StableHash for ($($idents),*,)
        where
            $($idents: $crate::traits::StableHash),*
        {
            fn stable_hash<H>(&self, hasher: &mut H)
            where
                H: ::std::hash::Hasher + Default,
            {
                #[allow(non_snake_case)]
                let ($(ref $idents),*,) = self;
                $(
                    $idents.stable_hash(&mut *hasher);
                )*
            }
        }
    };
}

macro_rules! transparent {
    () => {};
    (,) => {};
    (@!Sized $ty:ty) => {
        $crate::traits::macros::transparent!(@!Sized $ty,);
    };
    ($ty:ty) => {
        $crate::traits::macros::transparent!($ty,);
    };
    (@!Sized $ty:ty, $($tail:tt)*) => {
        impl crate::traits::StableHash for $ty {
            fn stable_hash<H>(&self, hasher: &mut H)
            where
                H: ::std::hash::Hasher
            {
                <$ty as ::std::hash::Hash>::hash(self, hasher)
            }
        }
        $crate::traits::macros::transparent!($($tail)*);
    };
    ($ty:ty, $($tail:tt)*) => {
        impl crate::traits::StableHash for $ty {
            fn stable_hash<H>(&self, hasher: &mut H)
            where
                H: ::std::hash::Hasher
            {
                <$ty as ::std::hash::Hash>::hash(self, hasher)
            }

            fn stable_hash_slice<H>(data: &[Self], hasher: &mut H)
            where
                Self: Sized,
                H: ::std::hash::Hasher,
            {
                <$ty as ::std::hash::Hash>::hash_slice(data, hasher)
            }
        }
        $crate::traits::macros::transparent!($($tail)*);
    };
}

pub(crate) use transparent;
pub(crate) use tup;
