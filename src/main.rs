use std::env;

use interface::Source;
use sources::syosetu;

// Mod
mod utils;
mod model;
mod interface;
mod epub;
mod sources;

fn main() {
    let args: Vec<String> = env::args().collect();
    let syosetu_source = syosetu::Syosetu {};
    let command = args[1].as_str();
    
    epub::gen_epub(syosetu_source.fetch_novel(command)).expect("Nothing");
}

