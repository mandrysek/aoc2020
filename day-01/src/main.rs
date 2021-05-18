use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const SUM: i32 = 2020;

fn main() {
    let mut numbers: Vec<i32> = vec![];

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines.flatten() {
            numbers.push(line.parse().unwrap());
        }
    }

    let size = numbers.len();

    for i in 0..size {
        for j in i + 1..size {
            for k in j + 1..size {
                if numbers[i] + numbers[j] + numbers[k] == SUM {
                    println!("Result is {}", numbers[i] * numbers[j] * numbers[k]);
                    return;
                }
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
