use sources::syosetu;

// use sources::example::Example;
use crate::interface::Source;
use std::env;

mod interface;
mod model;
mod sources;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].as_str();
    // let vip_novel = sources::vipnovel::VipNovel {};
    let syosetu = sources::syosetu::Syosetu {};
    println!("{:?}", &syosetu.fetching(command));
    
}
