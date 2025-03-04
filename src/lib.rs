use std::fs;
use std::{ thread, time };
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
    // search_single_thread(config.target, config.dir); 
    search_workers(config.target, config.dir, 4);
}

fn search_workers(target : String, dir : String, n_workers: usize) {
    assert_ne!(n_workers, 0);
    let heap : Arc<Mutex<BinaryHeap<String>>> = Arc::new(Mutex::new(BinaryHeap::new()));
    let target : Arc<String> = Arc::new(target);
    heap.lock().unwrap().push(dir);
    let mut workers = Vec::new();
    for _ in 0..n_workers {
        let heap_clone = Arc::clone(&heap);
        let target_clone = Arc::clone(&target);
        workers.push(thread::spawn( move || {
            loop {
                if let Some(current_dir) = { 
                    let mut lock = heap_clone.lock().unwrap();
                    lock.pop()
                } {
                    match fs::read_dir(&current_dir) {
                        Ok(contents) => {
                            for file in contents {
                                let file = file.unwrap();
                                let file_string = file.path().into_os_string().into_string().unwrap();
                                if file_string.contains(target_clone.as_str()) {
                                    println!("{}", &file_string);
                                }
                    
                                if file.metadata().unwrap().is_dir() {
                                    heap_clone.lock().unwrap().push(file_string);
                                }
                            }
                        },
                        Err(err) => {
                            eprintln!("Unable to read directory {current_dir}: {err}");
                        }
                    }
                } else {
                    break;
                }
            }
        }));
        thread::sleep(time::Duration::from_millis(10));
    }
    for worker in workers {
        worker.join();
    }
}

fn search_single_thread(target: String, dir: String) {
    let mut heap : BinaryHeap<String> = BinaryHeap::new();
    heap.push(dir);

    // Perform the search
    while let Some(current_dir) = heap.pop() {
        // Read the contents of the directory
        if let Ok(contents) = fs::read_dir(current_dir){
            for file in contents {
                let file = file.unwrap();
                let file_string = file.path().into_os_string().into_string().unwrap();
                if file_string.contains(target.as_str()) {
                    println!("{}", &file_string);
                }
    
                if file.metadata().unwrap().is_dir() {
                    heap.push(file_string);
                }
            }
        };
    }
}