extern crate "pgs-files" as pgs;

use pgs::passwd;

fn main() {
    let entry = passwd::get_entry_by_name("gary");
    match entry {
        Some(user) => {
            println!("{}: {} {} {}",
                     user.name, user.uid, user.dir, user.shell
            );
        },
        None => { println!("No such user!"); }
    };
}
