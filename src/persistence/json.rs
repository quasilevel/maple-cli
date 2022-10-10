use std::path;

use super::{MusicData, Storage};

pub struct JsonStorage {
    pub path: path::PathBuf,
    pub music_list: Vec<MusicData>,
}

impl Storage<MusicData> for JsonStorage {
    fn add(&self, _input: super::MusicInput) -> Result<MusicData, Box<dyn std::error::Error>> {
        todo!("implement the storage mechanism")
    }
}
