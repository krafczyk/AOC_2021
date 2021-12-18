extern crate argparse;
extern crate num;

use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;
use std::collections::HashSet;
use std::hash::Hash;
use std::ops::{Add,Sub,Mul};
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

#[derive(Debug,Hash,PartialEq,Eq)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn build(x: T, y:T) -> Point<T> {
        Point::<T> {x: x, y: y}
    }
}

enum Fold<T> {
    FoldX(T),
    FoldY(T),
}

fn process_input<P,T>(filename: P) -> (HashSet<Point<T>>, Vec<Fold<T>>)
where
    P: AsRef<Path>,
    T: Hash+PartialEq+Eq,
    T: Add+Add<Output=T>,
    T: Sub+Sub<Output=T>,
    T: FromStr,
    <T as FromStr>::Err: Debug
{
    let mut points: HashSet<Point<T>> = HashSet::new();
    let mut folds: Vec<Fold<T>> = Vec::new();
    // Reading the file
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if ip == "" {
                   continue
                }
                let tokens = ip.split_whitespace().collect::<Vec::<&str>>();
                if tokens[0] == "fold" {
                    let fold_vals: Vec<&str> = tokens[2].split('=').collect();
                    let fold_type = fold_vals[0];
                    let fold_val = fold_vals[1];
                    match fold_type {
                        "x" => {
                            folds.push(Fold::<T>::FoldX(fold_val.parse::<T>().unwrap()));
                        }
                        "y" => {
                            folds.push(Fold::<T>::FoldY(fold_val.parse::<T>().unwrap()));
                        }
                        _ => {
                            panic!("Encountered bad state!");
                        }
                    }
                } else {
                    let nums: Vec<&str> = ip.split(',').collect();
                    let x = nums[0].parse::<T>().unwrap();
                    let y = nums[1].parse::<T>().unwrap();
                    points.insert(Point::<T>::build(x,y));
                }
            }
        }
    } else {
		panic!("Problem opening file!");
	}
    (points, folds)
}

fn fold_point<T>(point: &Point<T>, fold: &Fold<T>) -> Point<T>
where
    T: Add+Add<Output=T>,
    T: Sub+Sub<Output=T>,
    T: Mul+Mul<Output=T>,
    T: num::NumCast,
    T: PartialOrd,
    T: Hash+PartialEq+Eq,
    T: Copy,
{
    match fold {
        Fold::<T>::FoldX(pos) => {
            let mut x = point.x;
            let y = point.y;
            if x > *pos {
                x = num::cast::<_,T>(2).unwrap()*(*pos)-x;
            }
            Point::<T>::build(x, y)
        },
        Fold::<T>::FoldY(pos) => {
            let x = point.x;
            let mut y = point.y;
            if y > *pos {
                y = num::cast::<_,T>(2).unwrap()*(*pos)-y;
            }
            Point::<T>::build(x, y)
        }
    }
}

fn fold_points<T>(points: &HashSet<Point<T>>, fold: &Fold<T>) -> HashSet<Point<T>>
where
    T: Add+Add<Output=T>,
    T: Sub+Sub<Output=T>,
    T: Mul+Mul<Output=T>,
    T: num::NumCast,
    T: PartialOrd,
    T: Hash+PartialEq+Eq,
    T: Copy,
{
    let mut new_points: HashSet<Point<T>> = HashSet::new();
    for p in points {
        new_points.insert(fold_point(p, fold));
    }
    new_points
}

fn solve_problem<T>(mut points: HashSet<Point<T>>, folds: &Vec<Fold<T>>)
where
    T: Add+Add<Output=T>,
    T: Sub+Sub<Output=T>,
    T: Mul+Mul<Output=T>,
    T: num::NumCast,
    T: PartialOrd,
    T: Hash+PartialEq+Eq,
    T: Copy,
    T: Ord,
    T: num::One,
{
    println!("Day 13 problem 1: {}", fold_points(&points, &folds[0]).len());

    // Do all folds
    for fold in folds {
        points = fold_points(&points, fold);
    }

    let max_x: T = points.iter().map(|p| p.x).max().unwrap();
    let max_y: T = points.iter().map(|p| p.y).max().unwrap();

    println!("Day 13 problem 2:");

    for y in num::range_inclusive::<T>(num::cast::<_,T>(0).unwrap(),max_y) {
        let mut line = String::new();
        for x in num::range_inclusive::<T>(num::cast::<_,T>(0).unwrap(), max_x) {
            let p = Point::<T>::build(x,y);
            if points.contains(&p) {
                line = line+"#";
            } else {
                line = line+" ";
            }
        }
        println!("{}", line);
    }
}

fn main() {
    // Argument Parsing
    let mut verbose = false;
    let mut input = String::new();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Day 13 AOC 2021");
        ap.refer(&mut input)
            .add_option(&["-i", "--input"], Store,
            "Filepath to input data");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue,
            "Add additional reporting");
        ap.parse_args_or_exit();
    }

    let (points, folds) = process_input::<_,i32>(input);

    solve_problem(points, &folds);
}
