use std::{error::Error, fs::File};

use crate::persistence::{json::JsonStorage, Music, Storage};

pub fn find_by_terms(terms: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut file = File::open("data.json")?;
    let storage = JsonStorage::open(&mut file)?;

    let list = storage.fuzzy_search(terms.join(" ").as_str())?;

    for music in list {
        println!("{}: {}", music.name()?, music.url()?);
    }
    Ok(())
}
