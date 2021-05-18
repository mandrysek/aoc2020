use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn main() {
    let mut valid_passwords: i32 = 0;
    let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines.flatten() {
            let groups = re.captures(&line).unwrap();
            let min: usize = groups[1].parse().unwrap();
            let max: usize = groups[2].parse().unwrap();
            let ch: char = groups[3].parse().unwrap();
            let password = String::from(&groups[4]);
            let ch1 = password.chars().nth(min - 1).unwrap();
            let ch2 = password.chars().nth(max - 1).unwrap();

            let valid = (if ch1 == ch { 1 } else { 0 }) + (if ch2 == ch { 1 } else { 0 }) == 1;

            if valid {
                valid_passwords += 1;
            }
        }
    }

    println!("Result is {}", valid_passwords);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
