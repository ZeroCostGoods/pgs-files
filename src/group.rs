use libc::types::os::arch::posix88::gid_t;

use entries::{Entries,Entry};


#[deriving(Show, PartialEq, PartialOrd)]
pub struct GroupEntry {

    /// Group Name
    name: String,

    /// Group Password
    passwd: String,

    /// Group ID
    gid: gid_t,

    /// Group Members
    members: Vec<String>,

}


impl Entry<GroupEntry> for GroupEntry {
    fn from_line(line: String) -> GroupEntry {

        let parts: Vec<&str> = line.as_slice().split_str(":").map(|part| part.trim()).collect();
        let members: Vec<String> = match parts[3].len() {
            0 => Vec::new(),
            _ => parts[3]
                .split_str(",")
                .map(|member| member.trim())
                .map(|member| member.to_string())
                .collect()
        };

        GroupEntry {
            name: parts[0].to_string(),
            passwd: parts[1].to_string(),
            gid: from_str(parts[2]).unwrap(),
            members: members,
        }
    }
}


pub fn get_entry_by_gid_from_path(path: &Path, gid: gid_t) -> Option<GroupEntry> {
    for entry in Entries::<GroupEntry>::new(path) {
        if entry.gid == gid {
            return Some(entry);
        }
    }
    None
}


pub fn get_entry_by_gid(gid: gid_t) -> Option<GroupEntry> {
    get_entry_by_gid_from_path(&Path::new("/etc/group"), gid)
}


pub fn get_entry_by_name_from_path(path: &Path, name: &str) -> Option<GroupEntry> {
    for entry in Entries::<GroupEntry>::new(path) {
        if entry.name.as_slice() == name {
            return Some(entry);
        }
    }
    None
}


pub fn get_entry_by_name(name: &str) -> Option<GroupEntry> {
    get_entry_by_name_from_path(&Path::new("/etc/group"), name)
}


pub fn get_all_entries_from_path(path: &Path) -> Vec<GroupEntry> {
    Entries::new(&Path::new(path)).collect()
}


pub fn get_all_entries() -> Vec<GroupEntry> {
    get_all_entries_from_path(&Path::new("/etc/group"))
}


#[test]
fn get_entry_by_gid_test() {
    let entry = get_entry_by_gid_from_path(&Path::new("testdata/group"), 0);
    assert!(entry.unwrap() == GroupEntry {
        name: "root".to_string(),
        passwd: "x".to_string(),
        gid: 0,
        members: Vec::<String>::new(),
    });

    let entry = get_entry_by_gid_from_path(&Path::new("testdata/group"), 4);
    assert!(entry.unwrap() == GroupEntry {
        name: "adm".to_string(),
        passwd: "x".to_string(),
        gid: 4,
        members: vec!["gary".to_string(), "root".to_string()],
    });

    let entry = get_entry_by_gid_from_path(&Path::new("testdata/group"), 666);
    assert!(entry == None);

}


#[test]
fn get_entry_by_name_test() {
    let entry = get_entry_by_name_from_path(&Path::new("testdata/group"), "root");
    assert!(entry.unwrap() == GroupEntry {
        name: "root".to_string(),
        passwd: "x".to_string(),
        gid: 0,
        members: vec![],
    });

    let entry = get_entry_by_name_from_path(&Path::new("testdata/group"), "adm");
    assert!(entry.unwrap() == GroupEntry {
        name: "adm".to_string(),
        passwd: "x".to_string(),
        gid: 4,
        members: vec!["gary".to_string(), "root".to_string()],
    });

    let entry = get_entry_by_name_from_path(&Path::new("testdata/group"), "zay");
    assert!(entry == None);

}


#[test]
fn get_all_entries_test() {
    let entries = get_all_entries_from_path(&Path::new("testdata/group"));
    let expected = vec![
        GroupEntry {
            name: "root".to_string(),
            passwd: "x".to_string(),
            gid: 0,
            members: vec![],
        },
        GroupEntry {
            name: "adm".to_string(),
            passwd: "x".to_string(),
            gid: 4,
            members: vec!["gary".to_string(), "root".to_string()],
        },
    ];

    assert_eq!(entries, expected);

}
