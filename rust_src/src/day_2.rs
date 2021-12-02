extern crate argparse;

use std::io::{self, BufRead};
use std::fs::File;
use std::ops::{Add,Sub,Mul};
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

struct LocationA<T>
where T: Add<Output = T>+Sub<Output = T>+Mul<Output = T>+Copy, {
    x: T,
    depth: T
}

struct LocationB<T>
where T: Add<Output = T>+Sub<Output = T>+Mul<Output = T>+Copy, {
    x: T,
    depth: T,
    aim: T,
}

enum Instructions<T>
where T: Add<Output = T>+Sub<Output = T>+Mul<Output = T>+Copy, {
    Forward(T),
    Down(T),
    Up(T)
}

impl<T> LocationA<T> where T: Add<Output = T>+Sub<Output = T>+Mul<Output = T>+Copy, {
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

impl<T> LocationB<T> where T: Add<Output = T>+Sub<Output = T>+Mul<Output = T>+Copy, {
    fn execute(&mut self, inst: &Instructions::<T>) {
        match inst {
            &Instructions::<T>::Forward(x) => {
                self.x = self.x + x;
                self.depth = self.depth + x*self.aim;
            },
            &Instructions::<T>::Down(d) => {
                self.aim = self.aim + d;
            },
            &Instructions::<T>::Up(d) => {
                self.aim = self.aim - d;
            }
        }
    }
}

fn read_instructions<P, T>(filename: P) -> Vec<Instructions<T>>
where T: Add<Output = T>+Sub<Output = T>+Mul<Output = T>+Copy+FromStr, P: AsRef<Path>, {
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

    let mut location_a = LocationA::<i32>{
        x:0,
        depth:0
    };

    let mut location_b = LocationB::<i32>{
        x:0,
        depth:0,
        aim:0,
    };

    for inst in instructions {
        location_a.execute(&inst);
        location_b.execute(&inst);
    }

    println!("Day 2 problem 1: {}", location_a.x*location_a.depth);
    println!("Day 2 problem 2: {}", location_b.x*location_b.depth);
}
