#![cfg_attr(not(test), no_std)]
#![feature(unsize)]
#![feature(coerce_unsized)]
#![feature(dispatch_from_dyn)]
#![feature(derive_coerce_pointee)]
#![feature(allocator_api)]

pub mod alloc;
pub mod error;
pub mod init;
pub mod list;
pub mod page;
pub mod str;
pub mod sync;
pub mod types;
