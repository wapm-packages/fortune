// #[macro_use]
// extern crate include_dir;

use std::fs;
use std::io;
use std::path::Path;
use rand::Rng;
use std::process;
use std::env;

// use include_dir::Dir;

// const PROJECT_DIR: Dir = include_dir!("src/data/");

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].to_lowercase().as_str() {
            "-h" | "--h" => {
                println!("available commands:");
                println!("\t-h                        this screen right here.");
                println!("\t-o <short,medium,long>    output short,medium or long quotes only.");
                process::exit(1);
            }
            "-o" | "--o" => {
                if args.len() > 2 {
                    match args[2].to_lowercase().as_str() {
                        "short" => get_quote("short"),
                        "medium" => get_quote("medium"),
                        "long" => get_quote("long"),
                        _ => println!("Use short, medium or long.")
                    }
                }
            }
            "-m" | "--m" => {
                unimplemented!();
            }
            _ => println!("No such command.")
        }
    } 
    else {
        get_quote("");
    }
    
    Ok(())
}

fn random(i: usize) -> usize {
    let mut r_thread = rand::thread_rng();
    r_thread.gen_range(0, i)
}

fn get_quote(quote_size: &str) {
    let file = read_file().unwrap();
    let quotes: Vec<&str> = file.split("\n%\n").collect();

    let mut tmp = vec![];
    let short = 150;
    let long = 400;

    match quote_size {
        "short" => {
            for q in &quotes {
                if q.len() <= short {
                    tmp.push(q)
                }
            }
            println!("{}", tmp[random(tmp.len())]);
        }
        "medium" => {
            for q in &quotes {
                if q.len() > short && q.len() < long {
                    tmp.push(q)
                }
            }
            println!("{}", tmp[random(tmp.len())]);
        }
        "long" => {
            for q in &quotes {
                if q.len() > long {
                    tmp.push(q)
                }
            }
            println!("{}", tmp[random(tmp.len())]);
        }
        _ => {
            println!("{}", quotes[random(quotes.len() - 1)]);
        }
    }
}

fn read_file() -> Result<String, &'static str> {
    return Ok(include_str!("data/fortunes").to_string())
    // let fmt = &format!("fortunes");
    // let file = PROJECT_DIR
    //     .get_file(&fmt)
    //     .expect(&format!("Can't find the fortunes file"));
    // let contents = str::from_utf8(file.contents).unwrap().to_string();
    // Ok(contents)
}
