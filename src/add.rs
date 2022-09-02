use std::error::Error;
use inquire::{Text, CustomType, Editor};
use url::Url;

pub fn add() -> Result<(), Box<dyn Error>> {
    let name = Text::new("Name:").prompt()?;
    let url = CustomType::<Url>::new("Url:").prompt()?;
    let description = Editor::new("Description:").prompt()?;
    println!("{} {} {}", name, url, description);
    Ok(())
}
