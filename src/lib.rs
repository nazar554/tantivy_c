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
                if !value.is_null() {
                    Box::from_raw(value);
                }
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

unsafe fn map_result<T: Copy + Default, E>(result: Result<T, E>, out_error: *mut *mut E) -> T {
    match result {
        Ok(value) => {
            if !out_error.is_null() {
                *out_error = std::ptr::null_mut();
            }
            value
        }
        Err(e) => {
            if !out_error.is_null() {
                *out_error = box_new_into_raw!(e);
            }
            T::default()
        }
    }
}

unsafe fn map_result_boxed<T, E>(result: Result<T, E>, out_error: *mut *mut E) -> *mut T {
    match result {
        Ok(value) => {
            if !out_error.is_null() {
                *out_error = std::ptr::null_mut();
            }
            box_new_into_raw!(value)
        }
        Err(e) => {
            if !out_error.is_null() {
                *out_error = box_new_into_raw!(e);
            }
            std::ptr::null_mut()
        }
    }
}

unsafe fn str_from_slice_parts<'a>(ptr: *const u8, len: usize) -> &'a str {
    debug_assert!(!ptr.is_null() || len == 0);
    let slice = std::slice::from_raw_parts(ptr, len);

    if cfg!(debug_assertions) {
        std::str::from_utf8(slice).unwrap()
    } else {
        std::str::from_utf8_unchecked(slice)
    }
}

dtor!(tantivy, error, tantivy::TantivyError);

unsafe fn write_or_truncate<T: Copy>(
    slice: &[T],
    buf: *mut T,
    len: *mut usize,
) {
    debug_assert!(!len.is_null());

    let slice_len = slice.len();

    let buffer_len = *len;
    *len = slice_len;

    if !buf.is_null() {
        let min_length = buffer_len.min(slice_len);

        let output = std::slice::from_raw_parts_mut(buf, min_length);
        output.copy_from_slice(&slice[..min_length]);
    }
}

use std::fmt::Display;

unsafe fn write_display_string<T: Display>(
    display: *const T,
    buf: *mut u8,
    len: *mut usize,
) {
    debug_assert!(!display.is_null());

    let bytes = (&*display).to_string().into_bytes();
    write_or_truncate(&bytes, buf, len)
}

#[no_mangle]
pub unsafe extern "C" fn tantivy_get_error_display_string(
    error: *const tantivy::TantivyError,
    buf: *mut u8,
    len: *mut usize,
) {
    write_display_string(&*error, buf, len)
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

mod query;
pub use self::query::*;
