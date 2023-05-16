use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

use crate::part1::part1;
use crate::part2::part2;

#[path = "day2/part1.rs"]
mod part1;
#[path = "day2/part2.rs"]
mod part2;

fn read_lines(filename: &str) -> io::Lines<BufReader<File>> {
    // Open the file in read-only mode.
    let file = File::open(filename).unwrap();
    // Read the file line by line, and return an iterator of the lines of the file.
    return BufReader::new(file).lines();
}

fn main() {
    day2();
}

fn day2() {
    let lines: Vec<String> = read_lines("day2.txt").map(|x| x.unwrap()).collect();
    part1(&lines);
    part2(&lines);
}
