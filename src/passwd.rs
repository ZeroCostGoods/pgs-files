use libc::types::os::arch::posix88::uid_t;
use libc::types::os::arch::posix88::gid_t;

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
