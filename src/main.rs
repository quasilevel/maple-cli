use clap::Parser;

#[derive(clap::Parser)]
struct Args {
   #[clap(subcommand)]
   action: Action,
}

#[derive(clap::Subcommand, Debug)]
enum Action {
   Add,
   Play,
   Find,
   Sync
}

fn main() {
    let args = Args::parse();

    println!("{:?} selected", args.action);
}
