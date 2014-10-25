use libc::types::os::arch::posix88::gid_t;

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
