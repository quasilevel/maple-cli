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
        serde_json::to_writer_pretty(dest, &self.music_list)?;
        Ok(())
    }
}

impl Storage<MusicData> for JsonStorage {
    fn add(&mut self, input: super::MusicInput) -> super::Result<MusicData> {
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

    fn get_tags(&self) -> super::Result<Vec<super::Tag>> {
        let all = self.music_list.iter().flat_map(|music| music.tags.iter());
        let mut vec = match all.size_hint() {
            (_, Some(size)) | (size, None) => Vec::with_capacity(size),
        };

        for tag in all {
            if !vec.contains(tag) {
                vec.push(tag.to_owned())
            }
        }

        Ok(vec)
    }

    fn search_with_tags(&self, tags: &[super::Tag]) -> super::Result<Vec<MusicData>> {
        todo!()
    }

    fn fuzzy_search(&self, search: &str) -> super::Result<Vec<MusicData>> {
        todo!()
    }

    fn get(&self, id: &str) -> super::Result<Option<MusicData>> {
        todo!()
    }

    fn update(&mut self, id: &str, input: super::MusicUpdate) -> super::Result<()> {
        todo!()
    }

    fn delete(&mut self, id: &str) -> super::Result<()> {
        todo!()
    }
}
