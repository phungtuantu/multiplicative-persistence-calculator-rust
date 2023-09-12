use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args{
    #[arg(short, long, default_value_t = 10)]
    base: u8,

    #[arg(short, long)]
    max_value: u64,
}

fn main() {
    let args = Args::parse();
    // TODO: Start getting stuffs to work in base 10
}
