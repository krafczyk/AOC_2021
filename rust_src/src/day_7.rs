extern crate argparse;
extern crate num;

use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;
use std::str::FromStr;
use std::fmt::Debug;
use argparse::{ArgumentParser, StoreTrue, Store};

// Define readlines function discussed here:
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn process_input<P, T>(filename: P) -> Vec<T>
where
    P: AsRef<Path>,
    T: FromStr+Debug,
    <T as FromStr>::Err: Debug,
{
    // Reading the file
    let mut result = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let mut nums: Vec<T> = ip.split(',').map(|s| s.parse::<T>().unwrap()).collect();
                result.append(&mut nums);
            }
        }
    } else {
		panic!("Problem opening file!");
	}
    result
}

fn main() {
    // Argument Parsing
    let mut verbose = false;
    let mut input = String::new();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Day 7 AOC 2021");
        ap.refer(&mut input)
            .add_option(&["-i", "--input"], Store,
            "Filepath to input data");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue,
            "Add additional reporting");
        ap.parse_args_or_exit();
    }

    let positions = process_input::<_,i32>(input);

    let min_pos = positions.iter().min().unwrap();
    let max_pos = positions.iter().max().unwrap();

    let possible_positions: Vec<i32> = num::range_inclusive::<i32>(*min_pos, *max_pos).collect();

    let fuel_cost1 = |pos:&i32| {
        //sum(map(p -> abs(p-pos), positions))
        positions.iter().map(|p:&i32| (*p-*pos).abs()).sum::<i32>()
    };

    let min_fuel1 = possible_positions.iter().map(|p| fuel_cost1(p)).min().unwrap();

    println!("Day 7 problem 1: {}", min_fuel1);

    let fuel_cost = |d: i32| {
        d*(d+1)/2
    };

    let fuel_cost2 = |pos:&i32| {
        //sum(map(p -> abs(p-pos), positions))
        positions.iter().map(|p:&i32| fuel_cost((*p-*pos).abs())).sum::<i32>()
    };

    let min_fuel2 = possible_positions.iter().map(|p| fuel_cost2(p)).min().unwrap();

    println!("Day 7 problem 2: {}", min_fuel2);

}
