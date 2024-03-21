use std::env::{self};

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    println!("{}", query);
}
