extern crate argparse;
extern crate num;

use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;
use std::ops::{Add,AddAssign};
use std::cmp::PartialOrd;
use std::fmt::Debug;
use argparse::{ArgumentParser, StoreTrue, Store};

// Define readlines function discussed here:
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn process_input<P,T>(filename: P) -> Vec<Vec<T>>
where
    P: AsRef<Path>,
    T: num::NumCast,
{
    let mut result: Vec<Vec<T>> = Vec::new();
    // Reading the file
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let mut num_line: Vec<T> = Vec::new();
                for ch in ip.chars() {
                    num_line.push(num::cast::<_,T>(ch.to_digit(10).unwrap()).unwrap());
                }
                result.push(num_line);
            }
        }
    } else {
		panic!("Problem opening file!");
	}
    result
}

//fn print_type_of<T>(_: &T) {
//    println!("{}", std::any::type_name::<T>())
//}

struct Point {
    i: usize,
    j: usize
}

fn do_step<T>(the_map: &mut Vec<Vec<T>>) -> usize
where
    T: Add+Add<Output=T>,
    T: AddAssign,
    T: num::NumCast,
    T: PartialOrd,
    T: Copy,
    T: Debug,
{
    let height = the_map.len();
    let width = the_map[0].len();
    let check_bounds = |p: &Point| -> bool {
        !(p.i < 1 || p.i > height || p.j < 1 || p.j > width)
    };

    let get_neighbors = |p: &Point| -> Vec<Point> {
        let i = p.i;
        let j = p.j;
        let ts = [(i+1,j+1),(i,j+1),(i-1,j+1),(i+1,j),(i-1,j),(i+1,j-1),(i,j-1),(i-1,j-1)];
        let ns = ts.iter().map(|t| Point { i: t.0, j: t.1 });
        return ns.filter(|n| check_bounds(n)).collect();
    };

    // First, increment all by one
    for v in the_map.iter_mut().flatten() {
        *v = *v+num::cast::<_,T>(1).unwrap();
    }

    // Second loop while there are flashers
    let mut num_flashes: usize = 0;
    while the_map.iter().flatten().any(|e| *e>num::cast::<_,T>(9).unwrap()) {
        for i in 1..(height+1) {
            for j in 1..(width+1) {
                let p = Point {i: i,j: j};
                if the_map[i-1][j-1] > num::cast::<_,T>(9).unwrap() {
                    // We have one ready to flash
                    let neighbors = get_neighbors(&p);
                    for n in neighbors {
                        let val = the_map[n.i-1][n.j-1];
                        if val != num::cast::<_,T>(0).unwrap() {
                            the_map[n.i-1][n.j-1] += num::cast::<_,T>(1).unwrap();
                        }
                    }
                    // Set the point to zero
                    the_map[i-1][j-1] = num::cast::<_,T>(0).unwrap();
                    num_flashes += 1;
                }
            }
        }
    }
    num_flashes
}

fn solve_problem<T>(the_map: &mut Vec<Vec<T>>)
where
	T: num::NumCast,
    T: PartialEq+PartialOrd,
    T: Add+Add<Output=T>,
    T: AddAssign,
    T: Copy,
    T: Debug,
{
    let mut flash_sequence: Vec<usize> = Vec::new();
    while !the_map.iter().flatten().all(|e| *e == num::cast::<_,T>(0).unwrap()) {
        let flashes = do_step(the_map);
        flash_sequence.push(flashes);
    }

    println!("Day 11 problem 1: {}", flash_sequence.iter().take(100).map(|v| *v).sum::<usize>());
    println!("Day 11 problem 2: {}", flash_sequence.len());
}

fn main() {
    // Argument Parsing
    let mut verbose = false;
    let mut input = String::new();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Day 11 AOC 2021");
        ap.refer(&mut input)
            .add_option(&["-i", "--input"], Store,
            "Filepath to input data");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue,
            "Add additional reporting");
        ap.parse_args_or_exit();
    }

    let mut the_map = process_input::<_,i32>(input);

    solve_problem::<i32>(&mut the_map);
}
