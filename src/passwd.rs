use libc::types::os::arch::posix88::uid_t;
use libc::types::os::arch::posix88::gid_t;

use entries::{Entries,Entry};

#[deriving(Show, PartialEq, PartialOrd)]
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


impl Entry<PasswdEntry> for PasswdEntry {
    fn from_line(line: String) -> PasswdEntry {

        let parts: Vec<&str> = line.as_slice().split_str(":").map(|part| part.trim()).collect();

        PasswdEntry {
            name: parts[0].to_string(),
            passwd: parts[1].to_string(),
            uid: from_str(parts[2]).unwrap(),
            gid: from_str(parts[3]).unwrap(),
            gecos: parts[4].to_string(),
            dir: parts[5].to_string(),
            shell: parts[6].to_string(),
        }
    }
}


pub fn get_entry_by_uid_from_path(path: &Path, uid: uid_t) -> Option<PasswdEntry> {
    for entry in Entries::<PasswdEntry>::new(path) {
        if entry.uid == uid {
            return Some(entry);
        }
    }
    None
}


pub fn get_entry_by_uid(uid: uid_t) -> Option<PasswdEntry> {
    get_entry_by_uid_from_path(&Path::new("/etc/passwd"), uid)
}


pub fn get_entry_by_name_from_path(path: &Path, name: &str) -> Option<PasswdEntry> {
    for entry in Entries::<PasswdEntry>::new(path) {
        if entry.name.as_slice() == name {
            return Some(entry);
        }
    }
    None
}


pub fn get_entry_by_name(name: &str) -> Option<PasswdEntry> {
    get_entry_by_name_from_path(&Path::new("/etc/passwd"), name)
}


pub fn get_all_entries_from_path(path: &Path) -> Vec<PasswdEntry> {
    Entries::new(&Path::new(path)).collect()
}


pub fn get_all_entries() -> Vec<PasswdEntry> {
    get_all_entries_from_path(&Path::new("/etc/passwd"))
}
