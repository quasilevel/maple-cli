use inquire::{CustomType, Editor, MultiSelect, Select, Text};
use std::{error::Error, fs::File};
use url::Url;

use crate::persistence::{json::JsonStorage, Storage};

const ADD_TAGS: &str = "Add tags";
const REMOVE_TAGS: &str = "Remove tags";
const DONE: &str = "Done";

pub fn add() -> Result<(), Box<dyn Error>> {
    let mut file = File::options()
        .read(true)
        .write(true)
        .create(true)
        .open("data.json")?;
    let mut storage = JsonStorage::open(&mut file).unwrap_or_default();

    let url = CustomType::<Url>::new("Url:").prompt()?;
    let name = Text::new("Name:").prompt()?;
    let description = Editor::new("Description:").prompt()?;
    let tags = input_tags(&["jazz", "english", "japanese"])?;

    storage.add(crate::persistence::MusicInput {
        url: url.into(),
        name,
        description,
        tags,
    })?;

    file.set_len(0)?;
    storage.store(&mut file)?;

    Ok(())
}

fn input_tags(suggestions: &[&str]) -> Result<Vec<String>, Box<dyn Error>> {
    let mut tags: Vec<String> = Vec::new();
    add_tags(&mut tags, suggestions)?;

    loop {
        let options: Vec<&str> = vec![ADD_TAGS, REMOVE_TAGS, DONE];
        let opt = Select::new("Choose an option", options).prompt()?;

        if opt == ADD_TAGS {
            add_tags(&mut tags, suggestions)?;
        } else if opt == REMOVE_TAGS {
            remove_tags(&mut tags)?;
        } else {
            break;
        }
    }

    Ok(tags)
}

fn add_tags(tags: &mut Vec<String>, suggestions: &[&str]) -> Result<(), Box<dyn Error>> {
    let suggester = |input: &str| -> Result<Vec<String>, inquire::CustomUserError> {
        let list: Vec<&str> = input.split(',').map(|x| x.trim()).collect();
        let (last, rest_vec) = list.split_last().unwrap_or((&"", &[""]));
        let rest = rest_vec.join(", ");
        let rest = match !rest.is_empty() {
            true => format!("{}, ", rest),
            false => rest,
        };

        Ok(suggestions
            .iter()
            .filter(|&val| !rest_vec.contains(val) && val.contains(&last.to_ascii_lowercase()))
            .map(|&val| format!("{}{}", rest.to_owned(), val))
            .collect())
    };

    let text = Text::new("Tags:")
        .with_help_message("Enter a comma separated list of tags")
        .with_suggester(&suggester)
        .prompt()?;

    tags.append(&mut text.split(',').map(|val| val.trim().to_string()).collect());
    Ok(())
}

fn remove_tags(tags: &mut Vec<String>) -> Result<(), Box<dyn Error>> {
    let removed = MultiSelect::new("Choose tags to remove:", tags.to_owned()).prompt()?;
    tags.retain(|tag| !removed.contains(tag));
    Ok(())
}
