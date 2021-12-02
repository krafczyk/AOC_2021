extern crate argparse;

use std::io::{self, BufRead};
use std::fs::File;
use std::ops::{Add,Sub};
use std::str::FromStr;
use std::path::Path;
use argparse::{ArgumentParser, StoreTrue, Store};
use regex::Regex;

// Define readlines function discussed here:
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Location<T>
where T: Add<Output = T>+Sub<Output = T>+Copy, {
    x: T,
    depth: T
}

enum Instructions<T>
where T: Add<Output = T>+Sub<Output = T>+Copy, {
    Forward(T),
    Down(T),
    Up(T)
}

impl<T> Location<T> where T: Add<Output = T>+Sub<Output = T>+Copy, {
    fn execute(&mut self, inst: &Instructions::<T>) {
        match inst {
            &Instructions::<T>::Forward(x) => {
                self.x = self.x + x;
            },
            &Instructions::<T>::Down(d) => {
                self.depth = self.depth + d;
            },
            &Instructions::<T>::Up(d) => {
                self.depth = self.depth - d;
            }
        }
    }
}

fn read_instructions<P, T>(filename: P) -> Vec<Instructions<T>>
where T: Add<Output = T>+Sub<Output = T>+Copy+FromStr, P: AsRef<Path>, {
    let re = Regex::new(r"(forward|down|up) ([0-9]*)").unwrap();
    let mut result = Vec::<Instructions::<T>>::new();
    // Reading the file
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                for cap in re.captures_iter(&ip) {
                    let instruction = &cap[1];
                    if let Ok(val) = &cap[2].parse::<T>() {
                        match instruction {
                            "forward" => result.push(Instructions::<T>::Forward(*val)),
                            "up" => result.push(Instructions::<T>::Up(*val)),
                            "down" => result.push(Instructions::<T>::Down(*val)),
                            _ => ()
                        }
                    }
                }
            }
        }
    }
    result
}

fn main() {
    // Argument Parsing
    let mut verbose = false;
    let mut input = String::new();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Day 2 AOC 2021");
        ap.refer(&mut input)
            .add_option(&["-i", "--input"], Store,
            "Filepath to input data");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue,
            "Add additional reporting");
        ap.parse_args_or_exit();
    }

    //read_instructions(input);
    let instructions: Vec::<Instructions::<i32>> = read_instructions(input);

    let mut location = Location::<i32>{
        x:0,
        depth:0
    };

    for inst in instructions {
        location.execute(&inst)
    }

    println!("Day 2 problem 1: {}", location.x*location.depth);
}
