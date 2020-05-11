//!Memory utilities
//!

use core::mem;

#[inline]
///Returns size of the value at compile time.
///
///Doesn't work with unsized types as they can be only evaluated at runtime.
pub const fn size_of<T: Sized>(_: &T) -> usize {
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
///
/////When dealing with references, one should remember that Rust doesn't collapse references
///fn test<T>(val: &T) -> usize {
///    size_of!(val,)
///}
///assert_eq!(test(&255u8), size_of!(usize)); // this will cause to return ptr size
///
///fn test2<T>(val: &T) -> usize {
///    //We use special syntax * to tell macro to not create extra reference
///    size_of!(*val)
///}
///assert_eq!(test2(&255u8), size_of!(u8));
///```
macro_rules! size_of {
    (*$val:expr) => {
        $crate::mem::size_of($val)
    };
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
///let array_ref = &array;
///
///assert_eq!(align_of!(array[0]), align_of!(u8));
///assert_eq!(align_of!(array.len()), align_of!(usize));
///assert_eq!(align_of!(array,), align_of!(u8));
///assert_eq!(align_of!(*array_ref), align_of!(u8));
///
///```
macro_rules! align_of {
    (*$val:expr) => {
        $crate::mem::align_of($val)
    };
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
///let string_ref = &string;
///
///assert!(!needs_drop!(array,));
///assert!(needs_drop!(string,));
///assert!(needs_drop!(*string_ref));
///assert!(!needs_drop!(string.as_bytes()[0]));
///```
macro_rules! needs_drop {
    (*$val:expr) => {
        $crate::mem::needs_drop($val)
    };
    ($type:ty) => {
        core::mem::needs_drop::<$type>()
    };
    ($val:expr$(,)?) => {
        $crate::mem::needs_drop(&$val)
    }
}
