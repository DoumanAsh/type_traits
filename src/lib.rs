//!Collection of utilities related to types and their properties

#![no_std]
#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::style))]

use core::{mem, marker};

///Type information
#[repr(transparent)]
pub struct Type<T>(marker::PhantomData<T>);

impl<T> Type<T> {
    ///Returns object size
    #[inline(always)]
    pub const fn size() -> usize {
        mem::size_of::<T>()
    }

    ///Returns minimum alignment
    #[inline(always)]
    pub const fn align() -> usize {
        mem::align_of::<T>()
    }

    #[inline(always)]
    ///Returns whether type is ZST
    pub const fn is_zst() -> bool {
        Self::size() == 0
    }

    #[inline(always)]
    ///Returns whether type has `Drop` implementation with side effects
    pub const fn needs_drop() -> bool {
        mem::needs_drop::<T>()
    }
}

///Static assertion helper
///
///This assertion relies on the fact that generic code is always compiled when generic is actually
///used, hence on its own every constant within `Assert` would not produce compile error, even if
///you refer to concrete instance of `Assert`
///In order to perform assertion, you must use associated constant, otherwise generic constant is not evaluated.
#[repr(transparent)]
pub struct Assert<T>(marker::PhantomData<T>);

impl<T> Assert<T> {
    ///Asserts type requires no call `Drop::drop`
    ///
    ///This relies on `mem::needs_drop` which may or may not return correctly, but if it returns
    ///`false` then it guarantees `Drop` has no side effect. Hence this assert should be used when
    ///you need to make sure you generic type has no `Drop` side effect.
    ///
    ///## Usage
    ///
    ///```
    ///use type_traits::Assert;
    ///
    ///fn test<T>(input: T) {
    ///    let _ = Assert::<T>::NO_NEED_DROP;
    ///}
    ///
    ///test(0);
    ///```
    pub const NO_NEED_DROP: () = assert!(!Type::<T>::needs_drop());

    ///Asserts type is not ZST.
    ///
    ///## Usage
    ///
    ///```
    ///use type_traits::Assert;
    ///
    ///fn test<T>(input: T) {
    ///    let _ = Assert::<T>::IS_NOT_ZST;
    ///}
    ///
    ///test(0);
    ///```
    pub const IS_NOT_ZST: () = assert!(!Type::<T>::is_zst());

    ///Asserts type is ZST
    ///
    ///## Usage
    ///
    ///```
    ///use type_traits::Assert;
    ///
    ///fn test<T>(input: T) {
    ///    let _ = Assert::<T>::IS_ZST;
    ///}
    ///
    ///test(());
    ///```
    pub const IS_ZST: () = assert!(Type::<T>::is_zst());
}

///Static assertion helper for pair of types
///
///This assertion relies on the fact that generic code is always compiled when generic is actually
///used, hence on its own every constant within `Assert` would not produce compile error, even if
///you refer to concrete instance of `Assert`
///In order to perform assertion, you must use associated constant, otherwise generic constant is not evaluated.
#[repr(transparent)]
pub struct Assert2<L, R>(marker::PhantomData<(L, R)>);

impl<L, R> Assert2<L, R> {
    ///Asserts both types are of the same size
    ///
    ///## Usage
    ///
    ///```
    ///use type_traits::Assert2;
    ///
    ///fn test<T, O>(input: T, default: O) -> O {
    ///    let _ = Assert2::<T, O>::IS_SAME_SIZE;
    ///    default
    ///}
    ///
    ///test(0u8, false);
    ///```
    pub const IS_SAME_SIZE: () = assert!(Type::<L>::size() == Type::<R>::size());

    ///Asserts both types are of the minimum alignment.
    ///
    ///## Usage
    ///
    ///```
    ///use type_traits::Assert2;
    ///
    ///fn test<T, O>(input: T, default: O) -> O {
    ///    let _ = Assert2::<T, O>::IS_SAME_ALIGN;
    ///    default
    ///}
    ///
    ///test(0u8, false);
    ///```
    pub const IS_SAME_ALIGN: () = assert!(Type::<L>::align() == Type::<R>::align());

    ///Asserts that `L` size is greater or equal to `R`
    ///
    ///## Usage
    ///
    ///```
    ///use type_traits::{Type, Assert2};
    ///
    ///fn test<T, O>(input: T, default: O) -> O {
    ///    assert!(Type::<T>::size() > Type::<O>::size());
    ///    let _ = Assert2::<T, O>::IS_LEFT_SIZE_GREATER_OR_EQUAL;
    ///    default
    ///}
    ///
    ///test(0u32, false);
    ///```
    pub const IS_LEFT_SIZE_GREATER_OR_EQUAL: () = assert!(Type::<L>::size() >= Type::<R>::size());

    ///Asserts that `L` size is less that of `R`
    ///
    ///## Usage
    ///
    ///```
    ///use type_traits::{Type, Assert2};
    ///
    ///fn test<T, O>(input: T, default: O) -> O {
    ///    assert!(Type::<T>::size() < Type::<O>::size());
    ///    let _ = Assert2::<T, O>::IS_LEFT_SIZE_LESS;
    ///    default
    ///}
    ///
    ///test(false, 0u32);
    ///```
    pub const IS_LEFT_SIZE_LESS: () = assert!(Type::<L>::size() < Type::<R>::size());

    ///Asserts that `L` minimum alignment is greater or equal to `R`
    ///
    ///## Usage
    ///
    ///```
    ///use type_traits::{Type, Assert2};
    ///
    ///fn test<T, O>(input: T, default: O) -> O {
    ///    assert!(Type::<T>::align() > Type::<O>::align());
    ///    let _ = Assert2::<T, O>::IS_LEFT_ALIGN_GREATER_OR_EQUAL;
    ///    default
    ///}
    ///
    ///test(0u32, false);
    ///```
    pub const IS_LEFT_ALIGN_GREATER_OR_EQUAL: () = assert!(Type::<L>::align() >= Type::<R>::align());

    ///Asserts that `L` minimum alignment is less that of `R`
    ///
    ///## Usage
    ///
    ///```
    ///use type_traits::{Type, Assert2};
    ///
    ///fn test<T, O>(input: T, default: O) -> O {
    ///    assert!(Type::<T>::align() < Type::<O>::align());
    ///    let _ = Assert2::<T, O>::IS_LEFT_ALIGN_LESS;
    ///    default
    ///}
    ///
    ///test(0u8, 0u32);
    ///```
    pub const IS_LEFT_ALIGN_LESS: () = assert!(Type::<L>::align() < Type::<R>::align());
}
