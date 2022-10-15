use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
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

fn has_all(outter: &Vec<String>, inner: &[String]) -> bool {
    inner
        .iter()
        .fold(true, |acc, item| acc && outter.contains(item))
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
        Ok(self
            .music_list
            .iter()
            .filter(|music| has_all(&music.tags, tags))
            .map(|music| music.to_owned())
            .collect())
    }

    fn fuzzy_search(&self, search: &str) -> super::Result<Vec<MusicData>> {
        let matcher = SkimMatcherV2::default().smart_case().use_cache(true);
        let mut weights = self
            .music_list
            .iter()
            .map(|music| {
                let weight = matcher.fuzzy_match(music.url.as_str(), search).unwrap_or(0)
                    + matcher
                        .fuzzy_match(music.name.as_str(), search)
                        .unwrap_or(0)
                    + matcher
                        .fuzzy_match(music.description.as_str(), search)
                        .unwrap_or(0);
                (music, weight)
            })
            .filter(|(_, weight)| *weight != 0)
            .collect::<Vec<_>>();

        weights.sort_by(|(_, a), (_, b)| a.cmp(b));

        Ok(weights
            .into_iter()
            .take(5)
            .map(|(music, _weight)| music.to_owned())
            .collect())
    }

    fn get(&self, id: &str) -> super::Result<Option<MusicData>> {
        Ok(self
            .music_list
            .iter()
            .find(|music| music.id == id)
            .and_then(|val| Some(val.to_owned())))
    }

    fn update(&mut self, id: &str, input: super::MusicUpdate) -> super::Result<()> {
        let music_data = match self.music_list.iter_mut().find(|music| music.id == id) {
            Some(data) => data,
            None => return Err("Music not found".into()),
        };

        if let Some(url) = input.url {
            music_data.url = url;
        }

        if let Some(name) = input.name {
            music_data.name = name;
        }

        if let Some(description) = input.description {
            music_data.description = description;
        }

        if let Some(tags) = input.tags {
            music_data.tags = tags;
        }

        Ok(())
    }

    fn delete(&mut self, id: &str) -> super::Result<()> {
        let idx = match self.music_list.iter().position(|music| music.id == id) {
            Some(idx) => idx,
            None => return Err("element not found".into()),
        };

        self.music_list.swap_remove(idx);

        Ok(())
    }
}
