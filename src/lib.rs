#![allow(non_snake_case)]

macro_rules! box_new_into_raw {
    ($e:expr) => {
        Box::into_raw(Box::new($e))
    };
}
macro_rules! ctor {
    ($prefix:ident, $name:ident, $t:ty, $e:expr) => {
        paste::item! {
            #[no_mangle]
                pub extern "C" fn [<$prefix _new_ $name>]() -> *mut $t {
                Box::into_raw(Box::new($e))
            }
        }
    };
}
macro_rules! dtor {
    ($prefix:ident, $name:ident, $t:ty) => {
        paste::item! {
            #[no_mangle]
            pub unsafe extern "C" fn [<$prefix _drop_ $name>](value: *mut $t) {
                Box::from_raw(value);
            }
        }
    };
}

macro_rules! ctor_dtor {
    ($prefix:ident, $name:ident, $t:ty, $e:expr) => {
        ctor!($prefix, $name, $t, $e);
        dtor!($prefix, $name, $t);
    };
}

unsafe fn str_from_slice_parts<'a>(ptr: *const u8, len: usize) -> &'a str {
    let slice = std::slice::from_raw_parts(ptr, len);
    debug_assert!(!ptr.is_null() || len == 0);

    if cfg!(debug_assertions) {
        std::str::from_utf8(slice).unwrap()
    } else {
        std::str::from_utf8_unchecked(slice)
    }
}

#[repr(C)]
pub struct Span<T> {
    ptr: *const T,
    len: usize,
}

impl<T> From<&[T]> for Span<T> {
    fn from(slice: &[T]) -> Span<T> {
        Span {
            ptr: slice.as_ptr(),
            len: slice.len(),
        }
    }
}

#[repr(C)]
pub struct SpanMut<T> {
    ptr: *mut T,
    len: usize,
}

impl<T> From<&mut [T]> for SpanMut<T> {
    fn from(slice: &mut [T]) -> SpanMut<T> {
        SpanMut {
            ptr: slice.as_mut_ptr(),
            len: slice.len(),
        }
    }
}

mod collector;
pub use self::collector::*;

mod schema;
pub use self::schema::*;

mod index;
pub use self::index::*;
