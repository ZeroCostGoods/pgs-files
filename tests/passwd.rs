extern crate pgs_files;

use std::path::Path;
use pgs_files::passwd;
use pgs_files::passwd::PasswdEntry;


#[test]
fn get_entry_by_uid_test() {
    let entry = passwd::get_entry_by_uid_from_path(&Path::new("testdata/passwd"), 0);
    assert!(entry.unwrap() == PasswdEntry {
        name: "root".to_string(),
        passwd: "x".to_string(),
        uid: 0,
        gid: 0,
        gecos: "root".to_string(),
        dir: "/root".to_string(),
        shell: "/bin/bash".to_string(),
     });

    let entry = passwd::get_entry_by_uid_from_path(&Path::new("testdata/passwd"), 1000);
    assert!(entry.unwrap() == PasswdEntry {
        name: "gary".to_string(),
        passwd: "x".to_string(),
        uid: 1000,
        gid: 1000,
        gecos: "Gary Josack,,,".to_string(),
        dir: "/home/gary".to_string(),
        shell: "/bin/bash".to_string(),
     });

    let entry = passwd::get_entry_by_uid_from_path(&Path::new("testdata/passwd"), 666);
    assert!(entry == None);

}


#[test]
fn get_entry_by_name_test() {
    let entry = passwd::get_entry_by_name_from_path(&Path::new("testdata/passwd"), "root");
    assert!(entry.unwrap() == PasswdEntry {
        name: "root".to_string(),
        passwd: "x".to_string(),
        uid: 0,
        gid: 0,
        gecos: "root".to_string(),
        dir: "/root".to_string(),
        shell: "/bin/bash".to_string(),
    });

    let entry = passwd::get_entry_by_name_from_path(&Path::new("testdata/passwd"), "gary");
    assert!(entry.unwrap() == PasswdEntry {
        name: "gary".to_string(),
        passwd: "x".to_string(),
        uid: 1000,
        gid: 1000,
        gecos: "Gary Josack,,,".to_string(),
        dir: "/home/gary".to_string(),
        shell: "/bin/bash".to_string(),
    });

    let entry = passwd::get_entry_by_name_from_path(&Path::new("testdata/passwd"), "zay");
    assert!(entry == None);

}


#[test]
fn get_all_entries_test() {
    let entries = passwd::get_all_entries_from_path(&Path::new("testdata/passwd"));
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
