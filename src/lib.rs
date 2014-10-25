#![crate_name = "pgs-files"]

// TODO(gary): Remove this when library is more developed.
#![allow(dead_code)]

extern crate libc;

mod passwd;
mod group;
mod shadow;
