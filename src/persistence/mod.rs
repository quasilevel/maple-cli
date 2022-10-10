use std::error::Error;

use serde::{Deserialize, Serialize};

pub mod json;

pub type Tag = String;

pub trait Music {
    fn id(&self) -> String;
    fn url(&self) -> String;
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn tags(&self) -> Vec<String>;
    fn added_on(&self) -> i64;
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
    fn id(&self) -> String {
        self.id.to_owned()
    }

    fn url(&self) -> String {
        self.url.to_owned()
    }

    fn name(&self) -> String {
        self.name.to_owned()
    }

    fn description(&self) -> String {
        self.description.to_owned()
    }

    fn tags(&self) -> Vec<Tag> {
        self.tags.to_owned()
    }

    fn added_on(&self) -> i64 {
        self.added_on
    }
}

pub struct MusicInput {
    pub url: String,
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
}

pub trait Storage<M: Music> {
    fn add(&mut self, input: MusicInput) -> Result<M, Box<dyn Error>>;
}
