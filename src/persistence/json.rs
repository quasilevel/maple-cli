use std::io::{Read, Write};
use uuid::Uuid;

use super::{MusicData, Storage};

#[derive(Default)]
pub struct JsonStorage {
    music_list: Vec<MusicData>,
}

impl JsonStorage {
    pub fn open(src: &mut impl Read) -> Result<Self, Box<dyn std::error::Error>> {
        let k: Vec<MusicData> = serde_json::from_reader(src)?;
        Ok(JsonStorage { music_list: k })
    }

    pub fn store(&self, dest: &mut impl Write) -> Result<(), Box<dyn std::error::Error>> {
        serde_json::to_writer(dest, &self.music_list)?;
        Ok(())
    }
}

impl Storage<MusicData> for JsonStorage {
    fn add(&mut self, input: super::MusicInput) -> Result<MusicData, Box<dyn std::error::Error>> {
        let data = MusicData {
            id: Uuid::new_v4().to_string(),
            url: input.url,
            name: input.name,
            description: input.description,
            tags: input.tags,
            added_on: chrono::offset::Utc::now().timestamp(),
        };

        self.music_list.push(data.clone());
        Ok(data)
    }
}
