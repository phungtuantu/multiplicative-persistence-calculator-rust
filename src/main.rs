use clap::Parser;
mod persistence_calculator;

use persistence_calculator::calculate_multiplicative_persistence;

#[derive(Parser)]
struct Args{
    #[arg(short, long, default_value_t = 10)]
    base: u64,

    #[arg(short, long)]
    max_value: u64,
}

fn main() {
    let args = Args::parse();
    let max_value = args.max_value;
    let base = args.base;
    println!("{}",calculate_multiplicative_persistence(max_value, base))
}
