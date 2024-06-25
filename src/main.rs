use std::{env, fs};

use syosetu::fetch_novel;
mod utils;
mod syosetu;
mod epub;
mod file;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].as_str();
    let novel = fetch_novel(command);
    epub::gen_epub(command, novel.1.as_str(), novel.0.as_str());
    utils::zip_them_all(format!("{}", command).as_str(), format!("{}.epub", declutter(novel.1)).as_str()).expect("Please");
    fs::remove_dir_all(format!("{}", command).as_str()).expect("Something");
}

fn declutter(name: String) -> String{
    let mut new_name = name;
    new_name.retain(|c| !c.is_whitespace());
    new_name
}
