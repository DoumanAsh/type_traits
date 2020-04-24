//!Memory utilities
//!

use core::mem;

#[inline]
///Returns size of the value at compile time.
///
///Doesn't work with unsized types as they can be only evaluated at runtime.
pub const fn size_of<T>(_: &T) -> usize {
    mem::size_of::<T>()
}

#[inline]
///Returns minimum alignment of the value at compile time..
///
///Doesn't work with unsized types as they can be only evaluated at runtime..
pub const fn align_of<T>(_: &T) -> usize {
    mem::align_of::<T>()
}

#[inline]
///Returns whether value requires a drop.
///
///Doesn't work with unsized types as they can be only evaluated at runtime..
pub const fn needs_drop<T>(_: &T) -> bool {
    mem::needs_drop::<T>()
}

#[macro_export]
///Returns size of supplied expression at compile time.
///
///Accepts both type and value.
///
///Doesn't work with unsized values
///
///## Usage
///
///```
///use type_traits::size_of;
///let array = [0u8; 4];
///
///assert_eq!(size_of!(array[0]), size_of!(u8));
///assert_eq!(size_of!(array.len()), size_of!(usize));
///```
///
///This will fail to compile because Rust macro system is actually just syntax replacement
///It is not aware about actual properties of passed arguments
///
///Anything that resembles type, will be tired as such
///As Rust lacks any utility to get type out of expression, you have to work it around by tricking
///macro into making something expression.
///
///```compile_fail
///use type_traits::size_of;
///let array = [0u8; 4];
///assert_eq!(size_of!(array), size_of!(u32));
///```
///
///Instead you can append colon to tell macro to treat it as value
///
///```
///use type_traits::size_of;
///let array = [0u8; 4];
///assert_eq!(size_of!(array,), size_of!(u32));
///```
macro_rules! size_of {
    ($type:ty) => {
        core::mem::size_of::<$type>()
    };
    ($val:expr$(,)?) => {
        $crate::mem::size_of(&$val)
    };
}

#[macro_export]
///Returns minimum alignment of the value at compile time..
///
///Accepts both type and value.
///
///Doesn't work with unsized values
///
///## Usage
///
///```
///use type_traits::align_of;
///let array = [0u8; 4];
///
///assert_eq!(align_of!(array[0]), align_of!(u8));
///assert_eq!(align_of!(array.len()), align_of!(usize));
///assert_eq!(align_of!(array,), align_of!(u8));
///
///```
macro_rules! align_of {
    ($type:ty) => {
        core::mem::align_of::<$type>()
    };
    ($val:expr$(,)?) => {
        $crate::mem::align_of(&$val)
    }
}

#[macro_export]
///Returns minimum alignment of the value at compile time..
///
///Accepts both type and value.
///
///Doesn't work with unsized values
///
///## Usage
///
///```
///use type_traits::needs_drop;
///let array = [0u8; 4];
///let string = "lolka".to_owned();
///
///assert!(!needs_drop!(array,));
///assert!(needs_drop!(string,));
///assert!(!needs_drop!(string.as_bytes()[0]));
///```
macro_rules! needs_drop {
    ($type:ty) => {
        core::mem::needs_drop::<$type>()
    };
    ($val:expr$(,)?) => {
        $crate::mem::needs_drop(&$val)
    }
}
