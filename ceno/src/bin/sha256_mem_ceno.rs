use clap::Parser;
use ceno::{prepare_sha256, prove};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long = "input-size")]
    input_size: usize,
}

fn main() {
    let args = Args::parse();
    let prepared = prepare_sha256(args.input_size);
    let _ = prove(&prepared);
}
