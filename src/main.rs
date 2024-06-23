// use sources::example::Example;
// use crate::interface::Source;

mod interface;
mod model;
mod sources;
mod utils;

use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let novel_arg = &args[1];

    let vip_novel = sources::vipnovel::VipNovel {};
}
