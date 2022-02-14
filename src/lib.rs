#![doc = include_str!("../README.md")]
#![no_std]
#![cfg_attr(
    const_impl,
    feature(const_trait_impl),
    feature(const_default_impls),
    feature(const_fn_trait_bound)
)]

#[cfg(feature = "const_default_impls")]
use const_fn::const_fn;
#[cfg(not(const_impl))]
use const_trait_impl::unconst_trait_impl;
use core::{default::Default, marker::PhantomData};
#[cfg(const_impl)]
use remove_macro_call::remove_macro_call;
pub use the_assoc_ty_ext::TheAssocTyExt;

/// [1-ZST](https://github.com/rust-lang/unsafe-code-guidelines/issues/172), i.e. one-aligned
/// zero-sized type.
/// 
/// # Example
/// ```rust
/// use zst::ZST;
/// use core::mem::{size_of, size_of_val};
/// 
/// // Repr is necessary to ensure the size of the discriminant
/// #[repr(u8)]
/// enum PrimUnsignedIntKinds {
///     U8(ZST<u8>),
///     U16(ZST<u16>),
///     U32(ZST<u32>),
///     U64(ZST<u64>),
///     U128(ZST<u128>),
///     Usize(ZST<usize>),
/// }
/// 
/// assert_eq!(size_of::<ZST<u16>>(), 0);
/// assert_eq!(
///     size_of_val(&PrimUnsignedIntKinds::U16(ZST::<u16>::default())),
///     size_of::<u8>()
/// );
/// // Since the ZST<T> is 1-aligned (#[repr(align(1))]), the following is guaranteed to hold
/// assert_eq!(
///     size_of::<PrimUnsignedIntKinds>(),
///     size_of::<core::mem::Discriminant<PrimUnsignedIntKinds>>()
/// );
/// ```
/// 
// Since ZST is both Eq and and PartialEq, it has structural match
// https://github.com/rust-lang/rust/issues/63438
#[derive(Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd, Copy)]
#[repr(align(1))]
pub struct ZST<T: ?Sized>(PhantomData<*const T>);

#[cfg_attr(const_impl, remove_macro_call)]
unconst_trait_impl! {
    impl<T: ?Sized> const Default for ZST<T> {
        fn default() -> Self {
            ZST(Default::default())
        }
    }
}

#[cfg_attr(const_impl, remove_macro_call)]
unconst_trait_impl! {
    impl<T: ?Sized> const TheAssocTyExt for ZST<T> {
        type TheAssocTy = T;
    }
}

impl<T: ?Sized> ZST<T> {
    #[cfg_attr(feature = "const_default_impls", const_fn)]
    #[inline(always)]
    pub fn new() -> ZST<T> {
        Default::default()
    }
}
