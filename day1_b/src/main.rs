use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::cmp;

fn get_total_fuel(mess: i64) -> i64 {
    let mut tmp = mess.clone();
    let mut total: i64 = 0;
    loop {
        let require = cmp::max(0, tmp / 3 - 2);
        if require == 0 {
            break
        }
        tmp = tmp / 3 - 2;
        total += require;
    }
    total
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let ret = reader
        .lines()
        .map(|s| s.unwrap().trim().parse::<i64>().unwrap())
        .map(get_total_fuel)
        .sum::<i64>();

    println!("{}", ret);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::get_total_fuel;

    #[test]
    fn text_example1() {
        let mass = 14;
        let ret = get_total_fuel(mass);
        assert_eq!(ret, 2);
    }

    #[test]
    fn test_example2() {
        let mass = 1969;
        let ret = get_total_fuel(mass);
        assert_eq!(ret, 966);
    }

    #[test]
    fn test_example3() {
        let mass = 100756;
        let ret = get_total_fuel(mass);
        assert_eq!(ret, 50346);
    }
}
