use libc::types::os::arch::c95::c_long;
use libc::types::os::arch::c95::c_ulong;

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
