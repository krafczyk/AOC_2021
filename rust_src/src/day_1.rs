extern crate argparse;
extern crate num;

use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;
use std::iter::Sum;
use num::Zero;
use argparse::{ArgumentParser, StoreTrue, Store};

// Define readlines function discussed here:
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn window_increases<'a, T>(numbers: &'a Vec<T>, window_size: usize) -> usize
where
    T: Ord+Sum<&'a T>+Zero
{
    let mut is_first: bool = true;
    let mut last_sum: T = T::zero();
    let mut num_increases: usize = 0;
    for i in 0..(numbers.len()-(window_size-1)) {
        let sum = numbers[i..i+window_size].iter().sum();
        if !is_first {
            if sum > last_sum {
                num_increases += 1;
            }
        }
        is_first = false;
        last_sum = sum;
    }
    return num_increases;
}

fn main() {
    // Argument Parsing
    let mut verbose = false;
    let mut input = String::new();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Day X AOC 2021");
        ap.refer(&mut input)
            .add_option(&["-i", "--input"], Store,
            "Filepath to input data");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue,
            "Add additional reporting");
        ap.parse_args_or_exit();
    }

    // Reading the file
    let mut numbers = Vec::new();
    if let Ok(lines) = read_lines(input) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let num: u32 = ip.trim().parse().expect("Not a number");
                numbers.push(num);
            }
        }
    }


    println!("Day 1: {} increases", window_increases(&numbers, 1));
    println!("Day 2: {} increases", window_increases(&numbers, 3));
}
