use std::fs;
use std::io;
use std::path::Path;
use rand::Rng;

fn main() -> io::Result<()> {

    let file = read_file().unwrap();

    let quotes: Vec<&str> = file.split("\n%\n").collect();

    let mut r_thread = rand::thread_rng();
    let r_num = r_thread.gen_range(0, quotes.len() - 1);

    for q in quotes[r_num].chars() {
        print!("{}", q);
    }

    Ok(())
}

fn read_file() -> Result<String, &'static str> {

    let quotebase = directory(std::path::Path::new("fortunes.txt")).unwrap();

    let file = match fs::read_to_string(quotebase) {
        Ok(f) => f,
        Err(_) => return Err("Could not open file"),
    };

    Ok(file)

}

fn directory<F: AsRef<Path>>(file: F) -> Result<std::path::PathBuf, &'static str> {
    
    let exe_path = match std::env::current_exe() {
        Ok(f) => f,
        Err(_) => return Err("Can't find executable"),
    };
    
    let exe_parent_path = match exe_path.parent() {
        Some(f) => f,
        None => return Err("Can't get executable's parent path."),
    };

    let path = exe_parent_path.join(file);

    Ok(path)

}