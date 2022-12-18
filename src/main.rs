mod args;

use args::RokuArgs;
use clap::Parser;

fn main() {
    let args = RokuArgs::parse();
    println!("{:?}", args);
}
