use std::fs::File;
use std::io::{self, BufRead, Lines, BufReader, Read};
use std::path::Path;
use std::iter::Flatten;

fn check_trees_hit(rows: Flatten<Lines<BufReader<File>>>, right: usize, down: usize) -> usize {
    let mut col_len: usize = 0;
    let mut trees: usize = 0;
    let mut col: usize = 0;

    for line in rows.step_by(down) {
        if col_len == 0 {
            col_len = line.len();
        }

        let char = line.chars().nth(col);

        match char {
            Some(char) => {
                if char == '#' {
                    trees += 1;
                }

                col += right;
            }
            _ => {}
        }

        col = col % col_len;
    }

    trees
}

fn main() {
    let mut trees: usize = 1;

    if let Ok(lines) = read_lines("./input.txt") {
        trees *= check_trees_hit(lines.flatten(), 1, 1);
    }

    if let Ok(lines) = read_lines("./input.txt") {
        trees *= check_trees_hit(lines.flatten(), 3, 1);
    }

    if let Ok(lines) = read_lines("./input.txt") {
        trees *= check_trees_hit(lines.flatten(), 5, 1);
    }

    if let Ok(lines) = read_lines("./input.txt") {
        trees *= check_trees_hit(lines.flatten(), 7, 1);
    }

    if let Ok(lines) = read_lines("./input.txt") {
        trees *= check_trees_hit(lines.flatten(), 1, 2);
    }

    println!("{:?}", trees);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
