use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let lines = lines_from_file("src/bin/day01/input.txt").unwrap();
    let lines: Vec<u32> = lines.iter().map(|d| d.parse::<u32>().unwrap()).collect();

    println!("{}", lines[0]);
    println!("n_increases: {}", n_increases(&lines));
    println!("sum_increases: {}", sum_increases(&lines));
}

fn n_increases(depths: &[u32]) -> usize {
    let mut increases = 0;
    for i in 1..(depths.len()) {
        if depths[i] > depths[i-1] {
            increases += 1;
        }
    }
    increases
}

fn sum_increases(depths: &[u32]) -> usize {
    let mut triplets = Vec::new();
    for i in 2..(depths.len()) {
        triplets.push(depths[i-2] + depths[i-1] + depths[i]);
    }
    n_increases(&triplets)
}


fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}