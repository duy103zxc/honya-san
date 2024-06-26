use std::env;
mod utils;
mod syosetu;
mod epub;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].as_str();
    epub::gen_epub(command).expect("Nothing");
}

