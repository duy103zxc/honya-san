use std::env;

use syosetu::fetch_novel;
mod zip;
mod utils;
mod syosetu;
mod epub;
mod file;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].as_str();
    let novel = fetch_novel(command);
    epub::gen_epub(command, novel.1.as_str(), novel.0.as_str());
}
