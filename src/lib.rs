use std::fs;
use std::thread;
use std::sync::{ Arc, Mutex };
use std::collections::BinaryHeap;

mod parse_args;
use parse_args::Args;

#[derive(Debug)]
pub struct Config {
    dir: String,
    target: String,
}

impl Config {
    pub fn build(
        args: impl Iterator<Item = String>
    ) -> Result<Config, &'static str> {

        let mut dir : String = String::from(".");

        // Parse the arguments provided into positional Arguments, Flags, and Keywork Arguments
        let mut parsed_args : Args;
        match Args::build(args) {
            Ok(ret) => {
                parsed_args = ret;
            },
            Err(reason) => return Err(reason),
        }

        for kw in parsed_args.keywords {
            match kw.keyword.as_str() {
                "--dir" => dir = kw.value,
                _ => return Err("Unknown keyword argument received."),
            }
        }

        let target = parsed_args.positional.remove(0);
        // Make sure dir is a valid directory
        if let Ok(file) = fs::metadata(&dir) {
            if !file.is_dir() {
                return Err("Search directory is not a directory");
            }
        } else {
            return Err("Can not find specified search directory.")
        }

        Ok(
            Config { target, dir, }
        )
    }
}

pub fn run(config : Config) {
    search_workers(config.target, config.dir, 1); 
}

fn search_workers(target : String, dir : String, workers: usize) {
    assert_ne!(workers, 0);
    let heap : Arc<Mutex<BinaryHeap<String>>> = Arc::new(Mutex::new(BinaryHeap::new()));
    heap.lock().unwrap().push(dir);

    // Perform the search
    while let Some(current_dir) = { 
        let mut heap_lock = heap.lock().unwrap();
        heap_lock.pop() 
    } {
        // Read the contents of the directory
        if let Ok(contents) = fs::read_dir(current_dir){
            for file in contents {
                let file = file.unwrap();
                let file_string = file.path().into_os_string().into_string().unwrap();
                if file_string.contains(target.as_str()) {
                    println!("{}", &file_string);
                }
    
                if file.metadata().unwrap().is_dir() {
                    heap.lock().unwrap().push(file_string);
                }
            }
        } else {
            eprintln!("Error reading directory");
        }
    }
    println!("Finished Searching.");   
}