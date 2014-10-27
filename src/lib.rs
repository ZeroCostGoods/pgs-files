//! A Library for parsing /etc/{passwd,group,shadow} files.

extern crate libc;

pub use entries::{Entries,Entry};

mod entries;

pub mod passwd;
pub mod group;
pub mod shadow;
