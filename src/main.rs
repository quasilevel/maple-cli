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
   Find {
       terms: Vec<String>,
   },
   Sync
}

#[derive(clap::Subcommand, Debug)]
enum Play {
    Title {
        title: String,
    },
    Tags {
        tags: Vec<String>,
    }
}

fn main() {
    let args = Args::parse();

    println!("{:?} selected", args.action);
}
