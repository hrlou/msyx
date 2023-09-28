use crate::prelude::*;

#[derive(Debug, Default)]
/// Representing an optomised data tree including all tracks
pub struct Database {
    // db: Option<sled::Db>,
    // scanners: Vec<Scanner>
}

impl Database {
    /// Travel through all files in a given directory
    pub fn new<T: Into<PathBuf>>(&mut self) -> Result<()> {
        todo!()
    }

    fn add<T: Into<PathBuf>>(&mut self, path: T) -> Result<()> {
        todo!()
    }

    // /// Consumes a Track and adds it to the Database
    // fn consume<T: Into<Track>>(&mut self, track: T) {
    // todo!()
    // }
}
