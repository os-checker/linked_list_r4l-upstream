#![cfg_attr(not(test), no_std)]
#![feature(unsize)]
#![feature(coerce_unsized)]
#![feature(dispatch_from_dyn)]
#![feature(derive_coerce_pointee)]
#![feature(allocator_api)]

/// Produces a pointer to an object from a pointer to one of its fields.
///
/// If you encounter a type mismatch due to the [`Opaque`] type, then use [`Opaque::cast_into`] or
/// [`Opaque::cast_from`] to resolve the mismatch.
///
/// [`Opaque`]: crate::types::Opaque
/// [`Opaque::cast_into`]: crate::types::Opaque::cast_into
/// [`Opaque::cast_from`]: crate::types::Opaque::cast_from
///
/// # Safety
///
/// The pointer passed to this macro, and the pointer returned by this macro, must both be in
/// bounds of the same allocation.
///
/// # Examples
///
/// ```
/// # use kernel::container_of;
/// struct Test {
///     a: u64,
///     b: u32,
/// }
///
/// let test = Test { a: 10, b: 20 };
/// let b_ptr: *const _ = &test.b;
/// // SAFETY: The pointer points at the `b` field of a `Test`, so the resulting pointer will be
/// // in-bounds of the same allocation as `b_ptr`.
/// let test_alias = unsafe { container_of!(b_ptr, Test, b) };
/// assert!(core::ptr::eq(&test, test_alias));
/// ```
#[macro_export]
macro_rules! container_of {
    ($field_ptr:expr, $Container:ty, $($fields:tt)*) => {{
        let offset: usize = ::core::mem::offset_of!($Container, $($fields)*);
        let field_ptr = $field_ptr;
        let container_ptr = field_ptr.byte_sub(offset).cast::<$Container>();
        $crate::assert_same_type(field_ptr, (&raw const (*container_ptr).$($fields)*).cast_mut());
        container_ptr
    }}
}

/// Helper for [`container_of!`].
#[doc(hidden)]
pub fn assert_same_type<T>(_: T, _: T) {}

pub mod alloc;
pub mod error;
pub mod init;
pub mod list;
pub mod page;
pub mod prelude;
pub mod str;
pub mod sync;
pub mod types;
