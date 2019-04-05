use std::fs;
use std::io;
use std::path::Path;
use rand::Rng;
use std::process;
use std::env;

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
                        "short" => quote_run("short"),
                        "medium" => quote_run("medium"),
                        "long" => quote_run("long"),
                        _ => println!("Use short, medium or long.")
                    }
                }
            }
            _ => println!("No such command.")
        }
    } 
    else {
        quote_run("all");
    }

    Ok(())
}

fn quote_run(i: &str) {
    let file = read_file().unwrap();
    let quotes: Vec<&str> = file.split("\n%\n").collect();

    let mut r_thread = rand::thread_rng();
    let mut r_num = r_thread.gen_range(0, quotes.len() -1);

    let mut tmp = vec![];

    match i {
        "short" => {
            for q in &quotes {
                if q.len() <= 150 {
                    tmp.push(q)
                }
            }
            r_num = r_thread.gen_range(0, tmp.len());
            println!("{}", tmp[r_num]);
        }
        "medium" => {
            for q in &quotes {
                if q.len() > 150 && q.len() < 400 {
                    tmp.push(q)
                }
            }
            r_num = r_thread.gen_range(0, tmp.len());
            println!("{}", tmp[r_num]);
        }
        "long" => {
            for q in &quotes {
                if q.len() > 400 {
                    tmp.push(q)
                }
            }
            r_num = r_thread.gen_range(0, tmp.len());
            println!("{}", tmp[r_num]);
        }
        _ => {
            println!("{}", quotes[r_num]);
        }
    }
}

fn read_file() -> Result<String, &'static str> {
    let quotebase = match directory("fortunes") {
        Ok(n) => n,
        Err(err) => {
            eprintln!("Error finding file: {}", err);
            process::exit(1);
        }
    };

    let file = match fs::read_to_string(quotebase) {
        Ok(f) => f,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            process::exit(1);
        }
    };

    Ok(file)
}

fn directory<F: AsRef<Path>>(file: F) -> Result<std::path::PathBuf, &'static str> {
    let exe_path = match std::env::current_exe() {
        Ok(f) => f,
        Err(_) => return Err("Could not find executable."),
    };
    
    let exe_parent_path = match exe_path.parent() {
        Some(f) => f,
        None => return Err("Can't get executable's parent path."),
    };

    let path = exe_parent_path.join(file);

    if path.exists() { 
        Ok(path) 
        } else {
            Err("Path not found.")
        }
}