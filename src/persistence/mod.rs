use serde::{Deserialize, Serialize};

pub mod json;

pub type Tag = String;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait Music {
    fn id(&self) -> Result<String>;
    fn url(&self) -> Result<String>;
    fn name(&self) -> Result<String>;
    fn description(&self) -> Result<String>;
    fn tags(&self) -> Result<Vec<String>>;
    fn added_on(&self) -> Result<i64>;
}

#[derive(Deserialize, Serialize, Clone)]
pub struct MusicData {
    id: String,
    url: String,
    name: String,
    description: String,
    tags: Vec<Tag>,
    added_on: i64,
}

impl Music for MusicData {
    fn id(&self) -> Result<String> {
        Ok(self.id.to_owned())
    }

    fn url(&self) -> Result<String> {
        Ok(self.url.to_owned())
    }

    fn name(&self) -> Result<String> {
        Ok(self.name.to_owned())
    }

    fn description(&self) -> Result<String> {
        Ok(self.description.to_owned())
    }

    fn tags(&self) -> Result<Vec<Tag>> {
        Ok(self.tags.to_owned())
    }

    fn added_on(&self) -> Result<i64> {
        Ok(self.added_on)
    }
}

pub struct MusicInput {
    pub url: String,
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
}

pub struct MusicUpdate {
    url: Option<String>,
    name: Option<String>,
    description: Option<String>,
    tags: Option<Vec<String>>,
}

pub trait Storage<M: Music> {
    fn get_tags(&self) -> Result<Vec<Tag>>;
    fn search_with_tags(&self, tags: &[Tag]) -> Result<Vec<M>>;
    fn fuzzy_search(&self, search: &str) -> Result<Vec<M>>;
    fn add(&mut self, input: MusicInput) -> Result<M>;
    fn get(&self, id: &str) -> Result<Option<M>>;
    fn update(&mut self, id: &str, input: MusicUpdate) -> Result<()>;
    fn delete(&mut self, id: &str) -> Result<()>;
}
