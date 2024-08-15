use clap::Parser;
use tempest::cli::Args;

fn main() {
    let args = Args::parse();

    println!("{:?}", args);
}
