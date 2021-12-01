extern crate argparse;

use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;
use argparse::{ArgumentParser, StoreTrue, Store};

// Define readlines function discussed here:
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
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
    let mut is_first: bool = true;
    let mut last_num: u32 = 0;
    let mut num_increases = 0;
    if let Ok(lines) = read_lines(input) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let num: u32 = ip.trim().parse().expect("Not a number");
                if !is_first {
                    if num > last_num {
                        num_increases += 1;
                    }
                }
                is_first = false;
                last_num = num;
            }
        }
    }
    println!("Day 1: {} increases", num_increases)
}
