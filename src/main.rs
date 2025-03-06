use std::env;
use win_search::{run, Config};

fn main() {
    match Config::build(env::args()) {
        Ok(config) => run(config),
        Err(msg) => {
            if msg != "" {
                eprintln!("Error: Invalid Configuration - {msg}")
            }
            eprintln!("

win-search - A Windows command line search tool

Usage: win-search --<key> <value> <-flag> target
Positional Arguments:
    target  :   The string that is being searched for
Optional Keyword Arguments:
    --dir <Directory name> : Specify the directory to begin the search in, defaults to the current directory
    --workers <Number of workers>   :   Specify the number of worker threads to spawn when searching, defaults to 4
Optional Flags:
    -ic :   Used to perform the search as case insensitive
    -h  :   Help (What you are reading now)
")
        }
    }
}
