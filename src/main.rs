extern crate bundler;

use std::env;
use std::process;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: bundle path/to/project");
        process::exit(1);
    }
    let code = bundler::bundle(&args[1]);
    println!("{}", code);
}
