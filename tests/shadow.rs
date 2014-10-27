extern crate pgs_files;

use pgs_files::shadow;
use pgs_files::shadow::ShadowEntry;



#[test]
fn get_entry_by_name_test() {
    let entry = shadow::get_entry_by_name_from_path(&Path::new("testdata/shadow"), "root");
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

    let entry = shadow::get_entry_by_name_from_path(&Path::new("testdata/shadow"), "zay");
    assert!(entry == None);

}


#[test]
fn get_all_entries_test() {
    let entries = shadow::get_all_entries_from_path(&Path::new("testdata/shadow"));
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
