use clap::Parser;

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

fn main() {
    let args = Args::parse();

    println!("{:?} selected", args.action);
}
