use std::io::{BufferedReader,File,IoResult};

/// Generic `Iterator` over implementor's of
/// [`Entry`](trait.Entry.html)'s.
///
/// # Examples
///
/// #### Iterate over /etc/passwd printing usernames
///
/// ```
/// for entry in Entries::<PasswdEntry>::new(&Path::new("/etc/passwd")) {
///     println!("{}", entry.name);
/// }
/// ```

pub struct Entries<T> {
    cursor: BufferedReader<IoResult<File>>,
}


impl<T> Entries<T> {
    pub fn new(file: &Path) -> Entries<T> {

        let reader = BufferedReader::new(
            File::open(file)
        );

        Entries { cursor: reader }
    }
}


impl<T: Entry<T>> Iterator<T> for Entries<T> {

    fn next(&mut self) -> Option<T> {
        let line = self.cursor.read_line();
        match line {
            Ok(_line) => Some(Entry::from_line(_line)),
            _ => None,
        }
    }

}

/// A Trait to represent an entry of data from an
/// /etc/{`passwd`,`group`,`shadow`} file.
pub trait Entry<T> {
    fn from_line(line: String) -> T;
}
