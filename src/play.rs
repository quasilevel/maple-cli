#[derive(clap::Subcommand, Debug)]
pub enum Play {
    Title {
        title: String,
    },
    Tags {
        tags: Vec<String>,
    }
}
