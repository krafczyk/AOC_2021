extern crate argparse;
extern crate num;

use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;
use std::str::FromStr;
use std::cmp::{PartialEq,Ord};
use std::cmp;
//use std::iter;
use std::ops::{Add,Sub};
use num::One;
use std::fmt::Debug;
//use std::fmt::{Debug,Display};
use std::collections::HashMap;
use std::hash::Hash;
use argparse::{ArgumentParser, StoreTrue, Store};
use regex::Regex;
use num::Signed;

// Define readlines function discussed here:
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug,Hash,Clone,Copy)]
struct Point<T> where T: Copy{
    x: T,
    y: T,
}

// Implement Eq for Point
impl<T> PartialEq for Point<T>
where
    T: PartialEq+Copy
{
    fn eq(&self, other: &Point<T>) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T> Eq for Point<T> where T: Eq+Copy {}

//impl<T> Hash for Point<T> where T: Hash {
//    fn hash<H: Hasher>(&self, state: &mut H) {
//        self.x.hash(state);
//        self.y.hash(state);
//    }
//}

#[derive(Debug,Clone,Copy)]
struct Line<T: Copy> {
    left: Point<T>,
    right: Point<T>,
}

fn process_input<P, T>(filename: P) -> Vec<Line<T>>
where
    P: AsRef<Path>,
    T: Copy+FromStr,
    <T as FromStr>::Err: Debug,
{
    let re = Regex::new(r"([0-9]*),([0-9]*) -> ([0-9]*),([0-9]*)").unwrap();
    let mut result: Vec<Line<T>> = Vec::new();
    // Reading the file
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                for cap in re.captures_iter(&ip) {
                    let x = &cap[1].parse::<T>().unwrap();
                    let y = &cap[2].parse::<T>().unwrap();
                    let left_point = Point::<T> {x: *x, y: *y};
                    let x = &cap[3].parse::<T>().unwrap();
                    let y = &cap[4].parse::<T>().unwrap();
                    let right_point = Point::<T> {x: *x, y: *y};
                    let line = Line::<T> {left: left_point, right: right_point};
                    result.push(line);
                }
            }
        }
    }
    return result
}

fn horiz_or_vert_line<T>(line: &Line<T>) -> Vec<Point<T>>
where
    T: PartialEq+Ord+One+Copy+Add+Add<Output=T>+num::ToPrimitive,
{
    let mut result: Vec<Point<T>> = Vec::new();
    let left = &line.left;
    let right = &line.right;
    if left.x == right.x {
        let x = left.x;
        let min_y = cmp::min(left.y, right.y);
        let max_y = cmp::max(left.y, right.y);
        for y in num::range_inclusive::<T>(min_y, max_y) {
            result.push(Point::<T> {x: x, y: y});
        }
    } else if left.y == right.y {
        let y = left.y;
        let min_x = cmp::min(left.x, right.x);
        let max_x = cmp::max(left.x, right.x);
        for x in num::range_inclusive::<T>(min_x, max_x) {
            result.push(Point::<T> {x: x, y: y});
        }
    }
    return result
}

fn horiz_vert_or_diag_line<T>(line: &Line<T>) -> Vec<Point<T>>
where
    T: PartialEq+Ord,
    T: One,
    T: Copy,
    T: Add+Add<Output=T>,
    T: num::ToPrimitive,
    T: Sub+Sub<Output=T>,
    T: Signed,
    T: Debug,
{
    let mut result: Vec<Point<T>> = Vec::new();
    let left = &line.left;
    let right = &line.right;
    if left.x == right.x {
        // Horizontal Line
        let x = left.x;
        let min_y = cmp::min(left.y, right.y);
        let max_y = cmp::max(left.y, right.y);
        for y in num::range_inclusive::<T>(min_y, max_y) {
            result.push(Point::<T> {x: x, y: y});
        }
    } else if left.y == right.y {
        // Vertical Line
        let y = left.y;
        let min_x = cmp::min(left.x, right.x);
        let max_x = cmp::max(left.x, right.x);
        for x in num::range_inclusive::<T>(min_x, max_x) {
            result.push(Point::<T> {x: x, y: y});
        }
    } else {
        // Diagonal Line
        let x_diff = right.x-left.x;
        let y_diff = right.y-left.y;
        if x_diff.abs() != y_diff.abs() {
            panic!("found line which isn't diagonal");
        }
        let num = x_diff.abs();
        let diffs: Vec<T> = num::range_inclusive::<T>(T::zero(), num).collect();
        let x_diffs: Vec<T>;
        if x_diff >= T::zero() {
            x_diffs = diffs.iter().cloned().collect();
        } else {
            x_diffs = diffs.iter().cloned().map(|v:T| -v).collect();
        }
        let y_diffs: Vec<T>;
        if y_diff >= T::zero() {
            y_diffs = diffs.iter().cloned().collect();
        } else {
            y_diffs = diffs.iter().cloned().map(|v:T| -v).collect();
        }
        let all_diffs: Vec<(T,T)> = x_diffs.iter().zip(y_diffs.iter()).map(|t| (*t.0, *t.1)).collect();
        let point_from_diff = |d: &(T,T)| {
            Point::<T>{ x: left.x+d.0, y: left.y+d.1 }
        };
        let points: Vec<Point<T>> = all_diffs.iter().map(point_from_diff).collect();
        for p in points {
            result.push(p);
        }
    }
    return result
}

fn compute_intersections<T>(points: &Vec<Point<T>>) -> usize
where
    T: PartialEq+Ord+One+Copy+Add+Add<Output=T>+num::ToPrimitive+Debug+Hash
{
    let position_count = &mut HashMap::<Point::<T>,usize>::new();

    let mut add_to_count = |p: &Point<T>| {
        //(&mut position_count).insert(*p.clone(), *position_count.entry(*p.clone()).or_insert(0)+1);
        *position_count.entry(*p).or_insert(0)+=1;
    };

    for p in points {
        add_to_count(&p);
    }

    let mut num = 0;
    for v in position_count.values() {
        if *v > 1 {
            num += 1;
        }
    }
    num
}

fn solve_problem_1<T>(lines: &Vec<Line<T>>)
where
    T: PartialEq+Ord+One+Copy+Add+Add<Output=T>+num::ToPrimitive+Debug+Hash
{
    let horiz_vert_selector = |l: & &Line<T>| {
        ((*l).left.x == (*l).right.x) || ((*l).left.y == (*l).right.y)
    };

    let lines: Vec<&Line<T>> = lines.iter().filter(horiz_vert_selector).collect();

    let mut points: Vec<Point<T>> = Vec::new();
    for line in &lines {
        let line_points = &mut horiz_or_vert_line(line);
        points.append(line_points);
    }

    let num = compute_intersections::<_>(&points);

    println!("Day 5 Problem 1: {}", num)
}

fn solve_problem_2<T>(lines: &Vec<Line<T>>)
where
    T: PartialEq+Ord+One+Copy+Add+Add<Output=T>+num::ToPrimitive+Debug+Hash+Signed
{
    let mut points: Vec<Point<T>> = Vec::new();
    for line in lines {
        let line_points = &mut horiz_vert_or_diag_line(line);
        points.append(line_points);
    }

    let num = compute_intersections::<_>(&points);

    println!("Day 5 Problem 2: {}", num)
}

fn main() {
    // Argument Parsing
    let mut verbose = false;
    let mut input = String::new();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Day 5 AOC 2021");
        ap.refer(&mut input)
            .add_option(&["-i", "--input"], Store,
            "Filepath to input data");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue,
            "Add additional reporting");
        ap.parse_args_or_exit();
    }

    let lines = process_input::<_,i32>(input);

    solve_problem_1::<i32>(&lines);
    solve_problem_2::<i32>(&lines);
}
