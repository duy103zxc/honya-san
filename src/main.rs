use std::env;

use interface::Source;
use sources::{kakuyomu, syosetu};

// Mod
mod utils;
mod model;
mod interface;
mod epub;
mod sources;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].as_str();
    let id = &args[2..].iter().cloned().collect::<Vec<_>>().join("");
    
    match command {
        "syosetu" => {
            let syosetu_source = syosetu::Syosetu {};
            epub::gen_epub(syosetu_source.fetch_novel(id)).expect("Nothing");
        }
        "kakuyomu" => {
            let kakuyomu_source = kakuyomu::Kakuyomu {};
            epub::gen_epub(kakuyomu_source.fetch_novel(id)).expect("Nothing");
   
        }
        _ => {
            println!("Something went wrong, please check the source name and source id before proceeding");
            std::process::exit(1);
        }
    }
}

