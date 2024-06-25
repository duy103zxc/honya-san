use std::env;
mod syosetu;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].as_str();

    let syosetu = syosetu::Syosetu {};
    println!("{:?}", &syosetu.fetching(command));  
}
