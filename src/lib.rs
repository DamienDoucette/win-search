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
    ignore_case: bool,
    workers: usize,
}

impl Config {
    pub fn build(
        args: impl Iterator<Item = String>
    ) -> Result<Config, &'static str> {

        let mut dir : String = String::from(".");
        let mut ignore_case : bool = false;
        let mut workers = 8;

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
                "--workers" => workers = kw.value.parse().unwrap(),
                _ => return Err("Unknown keyword argument received."),
            }
        }

        for flag in parsed_args.flags {
            match flag.as_str() {
                "-ic" => ignore_case = true,
                "-h" => return Err(""),
                _ => return Err("Unknown flag received"),
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
            Config { target, dir, ignore_case, workers }
        )
    }
}

pub fn run(config : Config) {
    if config.workers == 1 {
        search_single_thread(config);
    } else {
        search_workers(config);
    }
}

fn search_workers(config : Config) {
    assert_ne!(config.workers, 0);
    let heap : Arc<Mutex<BinaryHeap<String>>> = Arc::new(Mutex::new(BinaryHeap::new()));
    let target : Arc<String> = Arc::new(if config.ignore_case { config.target.to_lowercase() } else { config.target });
    heap.lock().unwrap().push(config.dir);
    let mut workers = Vec::new();
    for _ in 0..config.workers {
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
                                
                                if config.ignore_case {
                                    if file_string.to_lowercase().contains(target_clone.as_str()) {
                                        println!("{}", &file_string);
                                    }
                                } else {
                                    if file_string.contains(target_clone.as_str()) {
                                        println!("{}", &file_string);
                                    }
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
        worker.join().unwrap();
    }
}

fn search_single_thread(config: Config) {
    let mut heap : BinaryHeap<String> = BinaryHeap::new();
    heap.push(config.dir);
    let target = if config.ignore_case { config.target.to_lowercase() } else { config.target };

    // Perform the search
    while let Some(current_dir) = heap.pop() {
        // Read the contents of the directory
        if let Ok(contents) = fs::read_dir(current_dir){
            for file in contents {
                let file = file.unwrap();
                let file_string = file.path().into_os_string().into_string().unwrap();
                if config.ignore_case {
                    if file_string.to_lowercase().contains(target.as_str()) {
                        println!("{}", &file_string);
                    }
                } else {
                    if file_string.contains(target.as_str()) {
                        println!("{}", &file_string);
                    }
                }
    
                if file.metadata().unwrap().is_dir() {
                    heap.push(file_string);
                }
            }
        };
    }
}