use std::error::Error;
use inquire::{Text, CustomType, Editor, Select};
use url::Url;

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

    let options: Vec<&str> = vec!["remove tag(s)", "add tag(s)", "done"];
    let opt = Select::new("Choose an option", options).prompt()?;

    println!("{}", opt);

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
