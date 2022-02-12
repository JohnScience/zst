#![feature(prelude_import)]
#![feature(const_trait_impl)]
#![feature(const_default_impls)]
#![feature(const_fn_trait_bound)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use core::{default::Default, marker::PhantomData};
pub use the_assoc_ty_ext::TheAssocTyExt;
use const_trait_impl::unconst_trait_impl;
/// Generic type
pub struct ZST<T: ?Sized>(PhantomData<T>);
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::clone::Clone + ?Sized> ::core::clone::Clone for ZST<T> {
    #[inline]
    fn clone(&self) -> ZST<T> {
        match *self {
            ZST(ref __self_0_0) => ZST(::core::clone::Clone::clone(&(*__self_0_0))),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::fmt::Debug + ?Sized> ::core::fmt::Debug for ZST<T> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            ZST(ref __self_0_0) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "ZST");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::hash::Hash + ?Sized> ::core::hash::Hash for ZST<T> {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        match *self {
            ZST(ref __self_0_0) => ::core::hash::Hash::hash(&(*__self_0_0), state),
        }
    }
}
impl<T: ?Sized> ::core::marker::StructuralEq for ZST<T> {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::cmp::Eq + ?Sized> ::core::cmp::Eq for ZST<T> {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<PhantomData<T>>;
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::cmp::Ord + ?Sized> ::core::cmp::Ord for ZST<T> {
    #[inline]
    fn cmp(&self, other: &ZST<T>) -> ::core::cmp::Ordering {
        match *other {
            ZST(ref __self_1_0) => match *self {
                ZST(ref __self_0_0) => {
                    match ::core::cmp::Ord::cmp(&(*__self_0_0), &(*__self_1_0)) {
                        ::core::cmp::Ordering::Equal => ::core::cmp::Ordering::Equal,
                        cmp => cmp,
                    }
                }
            },
        }
    }
}
impl<T: ?Sized> ::core::marker::StructuralPartialEq for ZST<T> {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::cmp::PartialEq + ?Sized> ::core::cmp::PartialEq for ZST<T> {
    #[inline]
    fn eq(&self, other: &ZST<T>) -> bool {
        match *other {
            ZST(ref __self_1_0) => match *self {
                ZST(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &ZST<T>) -> bool {
        match *other {
            ZST(ref __self_1_0) => match *self {
                ZST(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::cmp::PartialOrd + ?Sized> ::core::cmp::PartialOrd for ZST<T> {
    #[inline]
    fn partial_cmp(&self, other: &ZST<T>) -> ::core::option::Option<::core::cmp::Ordering> {
        match *other {
            ZST(ref __self_1_0) => match *self {
                ZST(ref __self_0_0) => {
                    match ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)) {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                            ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                        }
                        cmp => cmp,
                    }
                }
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::marker::Copy + ?Sized> ::core::marker::Copy for ZST<T> {}
pub trait TraitName {}
impl<T: ?Sized> const TraitName for ZST<T> {}
impl<T: TraitName + ?Sized> const Default for ZST<T> {
    fn default() -> Self {
        ZST(Default::default())
    }
}
impl<T: TraitName + ?Sized> ZST<T> {
    #[inline(always)]
    pub fn new() -> ZST<T> {
        Default::default()
    }
}
