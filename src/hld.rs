use std::fs;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filename: String,
}

fn main() {
    let args = Args::parse();

    let contents = fs::read_to_string(args.filename).expect("Unable to read file");

    println!("File contents:\n{}", contents);
}
