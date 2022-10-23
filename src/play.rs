use std::{error::Error, fs::File};

use crate::persistence::{json::JsonStorage, Music, Storage};

#[derive(clap::Subcommand, Debug)]
pub enum Play {
    Title { title: String },
    Tags { tags: Vec<String> },
}

pub fn play(arg: Play) -> Result<(), Box<dyn Error>> {
    let mut file = File::open("data.json")?;
    let storage = JsonStorage::open(&mut file)?;

    let music_list = match arg {
        Play::Title { title: _ } => {
            unimplemented!("playing with title is not implemented")
        }
        Play::Tags { tags } => storage.search_with_tags(&tags)?,
    };

    let mut playlist_file = File::create("playlist.m3u")?;
    let mut playlist = m3u::EntryWriter::new(&mut playlist_file);

    for music in music_list.into_iter() {
        playlist.write_entry(&m3u::url_entry(&music.url()?).unwrap())?;
    }

    Ok(())
}
