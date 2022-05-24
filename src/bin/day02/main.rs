use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let lines = lines_from_file("src/bin/day02/input.txt").unwrap();
    println!("{}", lines[0]);
    println!("{}", x_times_z(&lines));
    println!("{}", part_two(&lines));
}

fn x_times_z(lines: &Vec<String>) -> u32 {
    let mut x = 0;
    let mut z = 0;
    let mut num: u32;
    for line in lines.iter() {
        num = line.chars().last().unwrap().to_digit(10).unwrap();
        match line.chars().next().unwrap() {
            'f' => x += num,
            'd' => z += num,
            'u' => z -= num,
            _ => panic!(),
        }
    }
    x * z
}

fn part_two(lines: &Vec<String>) -> u64 {
    let mut aim = 0;
    let mut x: u64 = 0;
    let mut z: u64 = 0;
    let mut num: u64;

    for line in lines.iter() {
        num = line.chars().last().unwrap().to_digit(10).unwrap() as u64;
        match line.chars().next().unwrap() {
            'f' => {
                x += num;
                z += aim * num;
            },
            'd' => aim += num,
            'u' => aim -= num,
            _ => panic!(),
        }
    }
    x * z
}

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}