use libc::types::os::arch::posix88::uid_t;
use libc::types::os::arch::posix88::gid_t;

use entries::{Entries,Entry};

#[deriving(Show, PartialEq, PartialOrd)]
pub struct PasswdEntry {
    /// Username
    name: String,

    /// User Password
    passwd: String,

    /// User ID
    uid: uid_t,

    /// Group ID
    gid: gid_t,

    /// User Information
    gecos: String,

    /// Home Directory
    dir: String,

    /// User's Shell
    shell: String,
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


#[test]
fn get_entry_by_uid_test() {
    let entry = get_entry_by_uid_from_path(&Path::new("testdata/passwd"), 0);
    assert!(entry.unwrap() == PasswdEntry {
        name: "root".to_string(),
        passwd: "x".to_string(),
        uid: 0,
        gid: 0,
        gecos: "root".to_string(),
        dir: "/root".to_string(),
        shell: "/bin/bash".to_string(),
     });

    let entry = get_entry_by_uid_from_path(&Path::new("testdata/passwd"), 1000);
    assert!(entry.unwrap() == PasswdEntry {
        name: "gary".to_string(),
        passwd: "x".to_string(),
        uid: 1000,
        gid: 1000,
        gecos: "Gary Josack,,,".to_string(),
        dir: "/home/gary".to_string(),
        shell: "/bin/bash".to_string(),
     });

    let entry = get_entry_by_uid_from_path(&Path::new("testdata/passwd"), 666);
    assert!(entry == None);

}


#[test]
fn get_entry_by_name_test() {
    let entry = get_entry_by_name_from_path(&Path::new("testdata/passwd"), "root");
    assert!(entry.unwrap() == PasswdEntry {
        name: "root".to_string(),
        passwd: "x".to_string(),
        uid: 0,
        gid: 0,
        gecos: "root".to_string(),
        dir: "/root".to_string(),
        shell: "/bin/bash".to_string(),
    });

    let entry = get_entry_by_name_from_path(&Path::new("testdata/passwd"), "gary");
    assert!(entry.unwrap() == PasswdEntry {
        name: "gary".to_string(),
        passwd: "x".to_string(),
        uid: 1000,
        gid: 1000,
        gecos: "Gary Josack,,,".to_string(),
        dir: "/home/gary".to_string(),
        shell: "/bin/bash".to_string(),
    });

    let entry = get_entry_by_name_from_path(&Path::new("testdata/passwd"), "zay");
    assert!(entry == None);

}


#[test]
fn get_all_entries_test() {
    let entries = get_all_entries_from_path(&Path::new("testdata/passwd"));
    let expected = vec![
        PasswdEntry {
            name: "root".to_string(),
            passwd: "x".to_string(),
            uid: 0,
            gid: 0,
            gecos: "root".to_string(),
            dir: "/root".to_string(),
            shell: "/bin/bash".to_string(),
        },
        PasswdEntry {
            name: "gary".to_string(),
            passwd: "x".to_string(),
            uid: 1000,
            gid: 1000,
            gecos: "Gary Josack,,,".to_string(),
            dir: "/home/gary".to_string(),
            shell: "/bin/bash".to_string(),
        },
    ];

    assert!(entries == expected);

}
