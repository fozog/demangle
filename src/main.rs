use rustc_demangle::demangle;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:#}", demangle(args[1].as_str()));
}
