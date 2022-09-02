use clap::Parser;

#[derive(clap::Parser)]
struct Args {
   #[clap(subcommand)]
   action: Action,
}

#[derive(clap::Subcommand, Debug)]
enum Action {
   Add,
   #[clap(subcommand)]
   Play(Play),
   Find,
   Sync
}

#[derive(clap::Subcommand, Debug)]
enum Play {
    Title {
        #[clap(value_parser)]
        title: String,
    },
    Tags {
        #[clap(value_parser)]
        tags: Vec<String>,
    }
}

fn main() {
    let args = Args::parse();

    println!("{:?} selected", args.action);
}
