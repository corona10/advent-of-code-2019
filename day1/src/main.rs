use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn get_fuel(mess: i64) -> i64 {
    mess / 3 - 2
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let ret = reader
        .lines()
        .map(|s| s.unwrap().trim().parse::<i64>().unwrap())
        .map(get_fuel)
        .sum::<i64>();

    println!("{}", ret);
    Ok(())
}
