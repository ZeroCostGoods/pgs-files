use libc::types::os::arch::c95::c_long;
use libc::types::os::arch::c95::c_ulong;

use entries::{Entries,Entry};


#[deriving(Show, PartialEq, PartialOrd)]
pub struct ShadowEntry {
    /// Login name
    name: String,

    /// Encrypted password
    passwd: String,

    /// Date of last change (measured in days since 1970-01-01 00:00:00 +0000 (UTC))
    last_change: c_long,

    /// Min number of days between changes
    min: c_long,

    /// Max number of days between changes
    max: c_long,

    /// Number of days before password expires to warn user to change it
    warning: c_long,

    /// Number of days after password expires until account is disabled
    inactivity: c_long,

    /// Date when account expires (measured
    /// in days since 1970-01-01 00:00:00 +0000 (UTC))
    expires: c_long,

    /// Reserved
    flag: c_ulong,
}


impl Entry<ShadowEntry> for ShadowEntry {
    fn from_line(line: String) -> ShadowEntry {

        let parts: Vec<&str> = line.as_slice().split_str(":").map(|part| part.trim()).collect();

        ShadowEntry {
            name: parts[0].to_string(),
            passwd: parts[1].to_string(),
            last_change: from_str(parts[2]).unwrap_or(-1),
            min: from_str(parts[3]).unwrap_or(-1),
            max: from_str(parts[4]).unwrap_or(-1),
            warning: from_str(parts[5]).unwrap_or(-1),
            inactivity: from_str(parts[6]).unwrap_or(-1),
            expires: from_str(parts[7]).unwrap_or(-1),
            flag: from_str(parts[8]).unwrap_or(0),
        }
    }
}


pub fn get_entry_by_name_from_path(path: &Path, name: &str) -> Option<ShadowEntry> {
    for entry in Entries::<ShadowEntry>::new(path) {
        if entry.name.as_slice() == name {
            return Some(entry);
        }
    }
    None
}


pub fn get_entry_by_name(name: &str) -> Option<ShadowEntry> {
    get_entry_by_name_from_path(&Path::new("/etc/shadow"), name)
}


pub fn get_all_entries_from_path(path: &Path) -> Vec<ShadowEntry> {
    Entries::new(&Path::new(path)).collect()
}


pub fn get_all_entries() -> Vec<ShadowEntry> {
    get_all_entries_from_path(&Path::new("/etc/shadow"))
}


#[test]
fn get_entry_by_name_test() {
    let entry = get_entry_by_name_from_path(&Path::new("testdata/shadow"), "root");
    assert!(entry.unwrap() == ShadowEntry {
        name: "root".to_string(),
        passwd: "!".to_string(),
        last_change: 16034,
        min: 0,
        max: 99999,
        warning: 7,
        inactivity: -1,
        expires: -1,
        flag: 0,
    });

    let entry = get_entry_by_name_from_path(&Path::new("testdata/shadow"), "zay");
    assert!(entry == None);

}


#[test]
fn get_all_entries_test() {
    let entries = get_all_entries_from_path(&Path::new("testdata/shadow"));
    let expected = vec![
        ShadowEntry {
            name: "root".to_string(),
            passwd: "!".to_string(),
            last_change: 16034,
            min: 0,
            max: 99999,
            warning: 7,
            inactivity: -1,
            expires: -1,
            flag: 0,
        },
        ShadowEntry {
            name: "gary".to_string(),
            passwd: "*".to_string(),
            last_change: 16034,
            min: 0,
            max: 99999,
            warning: 7,
            inactivity: -1,
            expires: -1,
            flag: 0,
        },
    ];

    assert!(entries == expected);

}
