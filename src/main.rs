use clap::Parser;
use std::error::Error;

mod add;
mod find;
mod persistence;
mod play;
mod player;
mod sync;

#[derive(clap::Parser)]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

#[derive(clap::Subcommand, Debug)]
enum Action {
    Add,
    #[clap(subcommand)]
    Play(play::Play),
    Find {
        terms: Vec<String>,
    },
    Sync,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.action {
        Action::Add => add::add(),
        Action::Play(arg) => play::play(arg),
        Action::Find { terms } => find::find_by_terms(terms),
        Action::Sync => panic!("Sync unimplemented"),
    }?;

    Ok(())
}
