use std::env;
use win_search::{run, Config};

fn main() {
    let conf = Config::build(env::args()).expect("Invalid Configuration");
    println!("{:#?}", &conf);
    run(conf);
}
