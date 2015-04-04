//! A Library for parsing /etc/{passwd,group,shadow} files.
//!
//! # Example Usage
//!
//! Print my `username`, `uid`, `homedir`, and `shell`
//!
//! ```
//! extern crate pgs_files;
//!
//! use pgs_files::passwd;
//!
//! fn main() {
//!     let entry = passwd::get_entry_by_name("gary");
//!     match entry {
//!         Some(user) => {
//!             println!("{}: {} {} {}",
//!                      user.name, user.uid, user.dir, user.shell
//!             );
//!         },
//!         None => { println!("No such user!"); }
//!     };
//! }
//! ```

extern crate libc;

pub use entries::{Entries,Entry};

mod entries;

pub mod passwd;
pub mod group;
pub mod shadow;
