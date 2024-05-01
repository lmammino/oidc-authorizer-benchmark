use clap::Parser;
use tokengen::TokenGen;
mod tokengen;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "10000")]
    iterations: usize,
    #[arg(short, long)]
    target_url: String,
}

fn main() {
    let args = Args::parse();
    let token_gen = TokenGen::default();

    for _ in 0..args.iterations {
        let token = token_gen.next_token();
        println!("GET {}\nAuthorization: Bearer {}\n", args.target_url, token);
    }
}
