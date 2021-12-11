extern crate argparse;
extern crate num;

use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;
use std::ops::{Add,Sub,AddAssign};
use std::cmp::PartialOrd;
use argparse::{ArgumentParser, StoreTrue, Store};
use std::collections::{HashSet,HashMap};
use std::hash::Hash;
use std::fmt::{Debug,Display};
use std::iter::FromIterator;

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
    let mut result = Vec::new();
    // Reading the file
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let mut line_nums: Vec<T> = Vec::new();
                for ch in ip.chars() {
                    line_nums.push(num::cast::<u32,T>(ch.to_digit(10).unwrap()).unwrap());
                }
                result.push(line_nums);
            }
        }
    } else {
		panic!("Problem opening file!");
	}
    result
}

#[derive(Debug,Hash,Eq,PartialEq,Clone,Copy)]
struct Point {
    i: i32,
    j: i32,
}

fn solve_problem<T>(the_map: Vec<Vec<T>>)
where
    T: Add+Add<Output=T>,
    T: AddAssign,
    T: Sub+Sub<Output=T>,
    T: PartialOrd,
    T: num::NumCast,
    T: Hash+PartialEq+Eq,
    T: Copy,
    T: Debug+Display,
{
    // Save map dimensions
    let height = num::cast::<_,i32>(the_map.len()).unwrap();
    let width = num::cast::<_,i32>(the_map[0].len()).unwrap();

    let check_bounds = |p: &Point| -> bool {
        ! (p.i < 0 || p.i >= height || p.j < 0 || p.j >= width)
    };

    let get_neighbors = |p: &Point| -> HashSet<Point> {
        let i = p.i;
        let j = p.j;
        let mut neighbors: HashSet<Point> = HashSet::new();
        let mut add_neighbor = |p: Point| {
            if check_bounds(&p) {
                neighbors.insert(p);
            }
        };
        for p in [
                (i+1,j),
                (i-1,j),
                (i,j+1),
                (i,j-1)] {
            let p = Point {i: p.0, j: p.1 };
            add_neighbor(p);
        }
        neighbors
    };

    let get_val = |p: &Point| -> T {
        return the_map[num::cast::<_,usize>(p.i).unwrap()][num::cast::<_,usize>(p.j).unwrap()]
    };

    let mut basins: Vec<Point> = Vec::new();

    let mut total_risk: T = num::cast::<_,T>(0).unwrap();
    for i in 0..height {
        for j in 0..width {
            let p = Point {i: i, j: j};
            let val = get_val(&p);
            let neighbors = get_neighbors(&p);
            if neighbors.iter().all(|p| get_val(p) > val) {
                // This is a basin. push it onto the list.
                basins.push(p);
                total_risk += val+num::cast::<_,T>(1).unwrap();
            }
        }
    }
    println!("Day 9 problem 1: {}", total_risk);

    // Calculate basin size. Here I'm using the apparent trick, that the edges of each basin are marked by tiles of amount 9.
    let mut basin_maps: HashMap<Point,HashSet<Point>> = HashMap::new();
    for basin_point in basins {
        // initialize visit list
        let mut visit = HashSet::<Point>::from_iter([basin_point]);
        let mut visited: HashSet<Point> = HashSet::new();
        let mut basin_elements: HashSet<Point> = HashSet::new();
        while visit.len() != 0 {
            // pop point from visit list
            let point = visit.iter().take(1).copied().collect::<Vec::<Point>>()[0];
            visit.remove(&point);
            // Get point neighbors
            let neighbors = get_neighbors(&point);
            // remove already visited points
            let neighbors: HashSet<_> = neighbors.difference(&visited.iter().copied().collect::<HashSet::<Point>>()).map(|n| *n).collect();
            for neighbor in &neighbors {
                let val = get_val(&neighbor);
                if val == num::cast::<_,T>(9).unwrap() {
                    // found edge point
                    visited.insert(*neighbor);
                } else {
                    // found point to add to visit
                    visit.insert(*neighbor);
                }
            }
            // Add point to the basin elements
            basin_elements.insert(point);
            visited.insert(point);
        }
        basin_maps.insert(basin_point, basin_elements);
    }

    let mut basin_sizes: Vec<usize> = basin_maps.values().map(|s| s.len()).collect::<Vec::<usize>>();
    basin_sizes.sort();
    basin_sizes.reverse();
    let largest_basins_prod: usize = basin_sizes[0..3].iter().fold(num::cast::<_,usize>(1).unwrap(), |acc, x| acc * x);
    println!("Day 9 problem 2: {}", largest_basins_prod)
}

fn main() {
    // Argument Parsing
    let mut verbose = false;
    let mut input = String::new();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Day 9 AOC 2021");
        ap.refer(&mut input)
            .add_option(&["-i", "--input"], Store,
            "Filepath to input data");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue,
            "Add additional reporting");
        ap.parse_args_or_exit();
    }

    let map_nums = process_input::<_,i32>(input);

    solve_problem::<_>(map_nums);
}
