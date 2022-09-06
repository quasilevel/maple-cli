use std::error::Error;
use inquire::{Text, CustomType, Editor, Select, MultiSelect};
use url::Url;

const ADD_TAGS: &str = "Add tags";
const REMOVE_TAGS: &str = "Remove tags";
const DONE: &str = "Done";

pub fn add() -> Result<(), Box<dyn Error>> {
    let name = Text::new("Name:").prompt()?;
    let url = CustomType::<Url>::new("Url:").prompt()?;
    let description = Editor::new("Description:").prompt()?;
    let tags = input_tags()?;
    println!("{} {} {} {:?}", name, url, description, tags);
    Ok(())
}

fn input_tags() -> Result<Vec<String>, Box<dyn Error>> {
    let mut tags: Vec<String> = Vec::new();
    add_tags(&mut tags)?;

    loop {
        let options: Vec<&str> = vec![ADD_TAGS, REMOVE_TAGS, DONE];
        let opt = Select::new("Choose an option", options).prompt()?;

        if opt == ADD_TAGS {
            add_tags(&mut tags)?;
        } else if opt == REMOVE_TAGS {
            remove_tags(&mut tags)?;
        } else {
            break
        }
    }

    Ok(tags)
}

fn add_tags(tags: &mut Vec<String>) -> Result<(), Box<dyn Error>> {
    loop {
        let val = Text::new("Tag:").with_help_message("Press enter to stop").prompt()?;
        if val.is_empty() {
            break;
        }
        tags.push(val)
    }
    Ok(())
}

fn remove_tags(tags: &mut Vec<String>) -> Result<(), Box<dyn Error>> {
    let removed = MultiSelect::new("Choose tags to remove:", tags.to_owned()).prompt()?;
    tags.retain(|tag| !removed.contains(tag));
    Ok(())
}
