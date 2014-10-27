extern crate pgs_files;

use pgs_files::group;
use pgs_files::group::GroupEntry;


#[test]
fn get_entry_by_gid_test() {
    let entry = group::get_entry_by_gid_from_path(&Path::new("testdata/group"), 0);
    assert!(entry.unwrap() == GroupEntry {
        name: "root".to_string(),
        passwd: "x".to_string(),
        gid: 0,
        members: Vec::<String>::new(),
    });

    let entry = group::get_entry_by_gid_from_path(&Path::new("testdata/group"), 4);
    assert!(entry.unwrap() == GroupEntry {
        name: "adm".to_string(),
        passwd: "x".to_string(),
        gid: 4,
        members: vec!["gary".to_string(), "root".to_string()],
    });

    let entry = group::get_entry_by_gid_from_path(&Path::new("testdata/group"), 666);
    assert!(entry == None);

}


#[test]
fn get_entry_by_name_test() {
    let entry = group::get_entry_by_name_from_path(&Path::new("testdata/group"), "root");
    assert!(entry.unwrap() == GroupEntry {
        name: "root".to_string(),
        passwd: "x".to_string(),
        gid: 0,
        members: vec![],
    });

    let entry = group::get_entry_by_name_from_path(&Path::new("testdata/group"), "adm");
    assert!(entry.unwrap() == GroupEntry {
        name: "adm".to_string(),
        passwd: "x".to_string(),
        gid: 4,
        members: vec!["gary".to_string(), "root".to_string()],
    });

    let entry = group::get_entry_by_name_from_path(&Path::new("testdata/group"), "zay");
    assert!(entry == None);

}


#[test]
fn get_all_entries_test() {
    let entries = group::get_all_entries_from_path(&Path::new("testdata/group"));
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
