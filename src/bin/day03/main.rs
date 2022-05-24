use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn main() {
    let lines = lines_from_file("src/bin/day03/input.txt").unwrap();
    let p1_epsilon = part_one(&lines, true);
    let p2_gamma = part_one(&lines, false);
    println!("{}", p1_epsilon * p2_gamma);
}


fn part_one(lines: &Vec<String>, epsilon: bool) -> u32 {
    let mut iter;
    let mut bit_sums = vec![0; lines[0].len()];
    let half_lines = lines.len() as u32 / 2;

    for line in lines {
        iter = line.chars().map(|c| c.to_digit(10).unwrap());
        for (i, bit) in iter.enumerate() {
            bit_sums[i] += bit;
        }
    }

    fn binary_finder(bit_sum: &u32, half_lines: u32, epsilon: bool) -> u32 {
        if epsilon {
            bit_sum / half_lines
        } else {
            half_lines / bit_sum
        }
    }

    let mut binary: String = "".to_owned();
    for char in bit_sums.iter().map(|sum| binary_finder(sum, half_lines, epsilon)) {
        binary.push_str(&char.to_string());
    }

    u32::from_str_radix(&binary, 2).expect("not a binary number!")
}


// 100000101101
// 011011010101
// 000000111000
// 110101110111
// 110000001100