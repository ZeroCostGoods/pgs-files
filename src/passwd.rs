//! Fuctions and Structs for dealing with /etc/passwd

use std::path::Path;
use std::num::ParseIntError;
use libc::uid_t;
use libc::gid_t;

use entries::{Entries,Entry};

/// An entry from /etc/passwd
#[derive(Debug, PartialEq, PartialOrd)]
pub struct PasswdEntry {
    /// Username
    pub name: String,

    /// User Password
    pub passwd: String,

    /// User ID
    pub uid: uid_t,

    /// Group ID
    pub gid: gid_t,

    /// User Information
    pub gecos: String,

    /// Home Directory
    pub dir: String,

    /// User's Shell
    pub shell: String,
}


impl Entry for PasswdEntry {
    fn from_line(line: &str) -> Result<PasswdEntry, ParseIntError> {

        let parts: Vec<&str> = line.split(":").map(|part| part.trim()).collect();

        Ok(PasswdEntry {
            name: parts[0].to_string(),
            passwd: parts[1].to_string(),
            uid: try!(parts[2].parse()),
            gid: try!(parts[3].parse()),
            gecos: parts[4].to_string(),
            dir: parts[5].to_string(),
            shell: parts[6].to_string(),
        })
    }
}


/// Return a [`PasswdEntry`](struct.PasswdEntry.html)
/// for a given `uid` and `&Path`
pub fn get_entry_by_uid_from_path(path: &Path, uid: uid_t) -> Option<PasswdEntry> {
    Entries::<PasswdEntry>::new(path).find(|x| x.uid == uid)
}


/// Return a [`PasswdEntry`](struct.PasswdEntry.html)
/// for a given `uid` from `/etc/passwd`
pub fn get_entry_by_uid(uid: uid_t) -> Option<PasswdEntry> {
    get_entry_by_uid_from_path(&Path::new("/etc/passwd"), uid)
}


/// Return a [`PasswdEntry`](struct.PasswdEntry.html)
/// for a given `name` and `&Path`
pub fn get_entry_by_name_from_path(path: &Path, name: &str) -> Option<PasswdEntry> {
    Entries::<PasswdEntry>::new(path).find(|x| x.name == name)
}


/// Return a [`PasswdEntry`](struct.PasswdEntry.html)
/// for a given `name` from `/etc/passwd`
pub fn get_entry_by_name(name: &str) -> Option<PasswdEntry> {
    get_entry_by_name_from_path(&Path::new("/etc/passwd"), name)
}


/// Return a `Vec<`[`PasswdEntry`](struct.PasswdEntry.html)`>` containing all
/// [`PasswdEntry`](struct.PasswdEntry.html)'s for a given `&Path`
pub fn get_all_entries_from_path(path: &Path) -> Vec<PasswdEntry> {
    Entries::new(path).collect()
}


/// Return a `Vec<`[`PasswdEntry`](struct.PasswdEntry.html)`>` containing all
/// [`PasswdEntry`](struct.PasswdEntry.html)'s from `/etc/passwd`
pub fn get_all_entries() -> Vec<PasswdEntry> {
    get_all_entries_from_path(&Path::new("/etc/passwd"))
}
