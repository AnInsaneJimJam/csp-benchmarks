use clap::Parser;
use ceno::{KeccakImpl, prepare_keccak, prove};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long = "input-size")]
    input_size: usize,
}

fn main() {
    let args = Args::parse();
    let prepared = prepare_keccak(args.input_size, KeccakImpl::CenoKeccak);
    let _ = prove(&prepared);
}
