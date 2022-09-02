use clap::Parser;
use std::error::Error;

mod add;
mod play;
mod find;
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
   Sync
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.action {
        Action::Add => println!("Add unimplemented"),
        Action::Play(_) => panic!("Play unimplemented"),
        Action::Find { terms: _ } => panic!("Find unimplemented"),
        Action::Sync => panic!("Sync unimplemented"),
    };

    Ok(())
}
