#![cfg_attr(feature = "structural_match", feature(structural_match))]
#![feature(const_trait_impl)]
#![feature(const_default_impls)]
#![feature(const_fn_trait_bound)]

use core::{
    default::Default,
    marker::PhantomData
};
pub use the_assoc_ty_ext::TheAssocTyExt;
use const_trait_impl::unconst_trait_impl;

/// Generic type 
// Since ZST is both Eq and and PartialEq, it has structural match
// https://github.com/rust-lang/rust/issues/63438
#[derive(Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd, Copy)]
pub struct ZST<T: ?Sized>(PhantomData<T>);

pub trait TraitName {}

impl<T: ?Sized> const TraitName for ZST<T> {}

#[unconst_trait_impl]
impl<T: ~const TraitName + ?Sized> const Default for ZST<T> {
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
