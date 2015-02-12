//! Fuctions and Structs for dealing with /etc/group

use libc::gid_t;

use entries::{Entries,Entry};


/// An entry from /etc/group
#[derive(Debug, PartialEq, PartialOrd)]
pub struct GroupEntry {

    /// Group Name
    pub name: String,

    /// Group Password
    pub passwd: String,

    /// Group ID
    pub gid: gid_t,

    /// Group Members
    pub members: Vec<String>,

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
            gid: parts[2].parse().unwrap(),
            members: members,
        }
    }
}


/// Return a [`GroupEntry`](struct.GroupEntry.html)
/// for a given `uid` and `&Path`
pub fn get_entry_by_gid_from_path(path: &Path, gid: gid_t) -> Option<GroupEntry> {
    Entries::<GroupEntry>::new(path).find(|x| x.gid == gid)
}


/// Return a [`GroupEntry`](struct.GroupEntry.html)
/// for a given `uid` from `/etc/group`
pub fn get_entry_by_gid(gid: gid_t) -> Option<GroupEntry> {
    get_entry_by_gid_from_path(&Path::new("/etc/group"), gid)
}


/// Return a [`GroupEntry`](struct.GroupEntry.html)
/// for a given `name` and `&Path`
pub fn get_entry_by_name_from_path(path: &Path, name: &str) -> Option<GroupEntry> {
    Entries::<GroupEntry>::new(path).find(|x| x.name.as_slice() == name)
}


/// Return a [`GroupEntry`](struct.GroupEntry.html)
/// for a given `name` from `/etc/group`
pub fn get_entry_by_name(name: &str) -> Option<GroupEntry> {
    get_entry_by_name_from_path(&Path::new("/etc/group"), name)
}


/// Return a `Vec<`[`GroupEntry`](struct.GroupEntry.html)`>` containing all
/// [`GroupEntry`](struct.GroupEntry.html)'s for a given `&Path`
pub fn get_all_entries_from_path(path: &Path) -> Vec<GroupEntry> {
    Entries::new(path).collect()
}


/// Return a `Vec<`[`GroupEntry`](struct.GroupEntry.html)`>` containing all
/// [`GroupEntry`](struct.GroupEntry.html)'s from `/etc/group`
pub fn get_all_entries() -> Vec<GroupEntry> {
    get_all_entries_from_path(&Path::new("/etc/group"))
}
