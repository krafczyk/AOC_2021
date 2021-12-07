extern crate argparse;
extern crate num;

use std::io::{self, BufRead};
use std::fs::File;
use std::str::FromStr;
use std::path::Path;
use std::fmt::Debug;
use std::ops::{AddAssign,Sub,Add};
use std::hash::Hash;
use std::collections::HashMap;
use argparse::{ArgumentParser, StoreTrue, Store};

// Define readlines function discussed here:
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_lifetimes<P,T>(filename: P) -> Vec<T>
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
        panic!("Problem opening file!")
    }
    return result;
}

fn build_pop_tracker<T>(init_lifetimes: &Vec<T>) -> HashMap<T,usize>
where
    T: Eq+Hash+AddAssign+Copy,
    T: num::ToPrimitive
{
    let mut pop_tracker = HashMap::<T,usize>::new();
    for lifetime in init_lifetimes {
        *pop_tracker.entry(*lifetime).or_insert(0)+=1;
    }
    return pop_tracker;
}

//fn print_type_of<T>(_: &T) {
//    println!("{}", std::any::type_name::<T>())
//}

fn do_step<T>(pop: &mut HashMap<T,usize>)
where
    T: PartialEq+Eq+Hash+Copy,
    T: num::ToPrimitive+num::Zero+num::One,
    T: num::NumCast,
    T: Debug,
    T: Ord,
    T: Sub+Sub<Output=T>,
{
    let num_reproducers: usize = *pop.entry(T::zero()).or_insert(0);
    let cur_lifetimes: Vec<T> = pop.keys().copied().collect();
    // Don't understand why I need to double dereference here..
    let mut cur_lifetimes2: Vec<&T> = cur_lifetimes.iter().filter(|&n| *n != T::zero()).collect::<Vec::<&T>>();
    cur_lifetimes2.sort();

    let max_lifetime = cur_lifetimes.iter().max().unwrap();

    // Move lifetimes down a notch.
    for &l in &cur_lifetimes2 {
        *pop.entry(*l-num::cast::<i32,T>(1).unwrap()).or_insert(0) = *pop.entry(*l).or_insert(0);
    }
    // Reset max lifetime,there are no more like it.
    pop.insert(*max_lifetime, 0);

    // Reproducers
    // The adults
    *pop.entry(num::cast::<i32,T>(6).unwrap()).or_insert(0) += num_reproducers;
    // The children
    *pop.entry(num::cast::<i32,T>(8).unwrap()).or_insert(0) += num_reproducers;
}

fn count_pop<T>(pop: &HashMap<T,usize>) -> usize
where
    T: Add+Add<Output=T>
{
     pop.values().sum()
}

fn main() {
    // Argument Parsing
    let mut verbose = false;
    let mut input = String::new();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Day 6 AOC 2021");
        ap.refer(&mut input)
            .add_option(&["-i", "--input"], Store,
            "Filepath to input data");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue,
            "Add additional reporting");
        ap.parse_args_or_exit();
    }

    let lifetimes = get_lifetimes::<_,u32>(input);

    let mut pop = build_pop_tracker::<u32>(&lifetimes);

    let mut pop_amounts: Vec<usize> = Vec::new();
    pop_amounts.push(count_pop(&pop));

    for _ in 0..256 {
        do_step(&mut pop);
        pop_amounts.push(count_pop(&pop));
    }

    println!("Day 6 problem 1: {}", pop_amounts[80]);
    println!("Day 6 problem 2: {}", pop_amounts[256]);
}
