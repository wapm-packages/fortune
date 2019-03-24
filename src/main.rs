use std::fs;
use std::env;
use std::io;
use std::path::Path;
use rand::Rng;

fn main() -> io::Result<()> {

    // Lazy fix for bug of not finding fortunes.txt when creating a symlink on linux.
    // hopepfully this should work on Windows too. Have not tried it yet.
    let tmp = std::env::current_exe()?;
    let tmp2 = tmp.parent().unwrap();
    let dir = Path::new(tmp2);
    let file = Path::new("fortunes.txt");
    let file_dir = dir.join(file);

    // Start reading, collecting and printing.
    let f = fs::read_to_string(file_dir)?;
    let quotes: Vec<&str> = f.split("\n%\n").collect();

    let mut random = rand::thread_rng();
    let r_num = random.gen_range(0, quotes.len() - 1);

    for q in quotes[r_num].chars() {
        print!("{}", q);
    }

    Ok(())
}
