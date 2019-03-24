use std::fs;
use std::io;
use rand::Rng;

fn main() -> io::Result<()> {
    let f = "fortunes";
    let f = fs::read_to_string(f)?;
    let quotes: Vec<&str> = f.split("\n%\n").collect();

    let mut random = rand::thread_rng();
    let r_num = random.gen_range(0, quotes.len() - 1);

    for q in quotes[r_num].chars() {
        print!("{}", q);
    }

    Ok(())
}
