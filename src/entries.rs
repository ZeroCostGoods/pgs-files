use std::io::{BufferedReader,File,IoResult};


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


pub trait Entry<T> {
    fn from_line(line: String) -> T;
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
