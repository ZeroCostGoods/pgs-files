/// Generic `Iterator` over implementor's of
/// [`Entry`](trait.Entry.html)'s.
///
/// # Examples
///
/// #### Iterate over /etc/passwd printing usernames
///
/// ```
/// use std::path::Path;
/// use pgs_files::passwd::PasswdEntry;
/// use pgs_files::Entries;
///
/// for entry in Entries::<PasswdEntry>::new(&Path::new("/etc/passwd")) {
///     println!("{}", entry.name);
/// }
/// ```

use std::io::{BufRead,BufReader};
use std::fs::File;
use std::path::Path;
use std::marker::PhantomData;
use std::num::ParseIntError;


pub struct Entries<T> {
    cursor: BufReader<File>,
    marker: PhantomData<T>,
}

impl<T> Entries<T> {
    pub fn new(file: &Path) -> Entries<T> {
        let reader = BufReader::new(File::open(file).ok().unwrap());
        Entries {
            cursor: reader,
            marker: PhantomData,
        }
    }
}


impl<T: Entry> Iterator for Entries<T> {

    type Item = T;

    fn next(&mut self) -> Option<T> {
        let mut line = String::new();
        loop {
            // We might need to make multiple loops to drain off
            // comment lines. Start with an empty string per loop.
            line.clear();
            match self.cursor.read_line(&mut line){
                Ok(0) => return None,
                Ok(_) => (),
                _     => return None,
            }

            if line.starts_with("#") {
                continue;
            }

            match T::from_line(&line) {
                Ok(entry) => return Some(entry),
                // Parse Error. Just ignore this entry.
                _         => (),
            }
        }
    }

}

/// A Trait to represent an entry of data from an
/// /etc/{`passwd`,`group`,`shadow`} file.
pub trait Entry: Sized {
    fn from_line(line: &str) -> Result<Self, ParseIntError>;
}
