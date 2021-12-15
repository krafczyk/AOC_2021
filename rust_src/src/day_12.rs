extern crate argparse;

use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;
use std::collections::{HashMap,HashSet};
use std::iter::FromIterator;
use argparse::{ArgumentParser, StoreTrue, Store};

// Define readlines function discussed here:
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn process_input<P>(filename: P) -> HashMap<String,HashSet<String>>
where
    P: AsRef<Path>,
{
    // Reading the file
    let mut result_map: HashMap<String,HashSet<String>> = HashMap::new();
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let pair: Vec<String> = ip.split('-').map(|s| String::from(s)).collect::<Vec::<String>>();
                result_map.entry(pair[0].clone()).or_insert(HashSet::<String>::new()).insert(pair[1].clone());
                result_map.entry(pair[1].clone()).or_insert(HashSet::<String>::new()).insert(pair[0].clone());
            }
        }
    } else {
		panic!("Problem opening file!");
	}
    result_map
}

fn num_paths1(cur_path: &Vec<&String>, cave_map: &HashMap<String,HashSet<String>>) -> usize {
    if cur_path[cur_path.len()-1] == "end" {
        // We made it to the end. this is a valid path.
        return 1;
    }
    //println!("cur_path: {:?}", cur_path);
    // Get room list of last room in the path
    let possible_next_steps: &HashSet<String> = cave_map.get(cur_path[cur_path.len()-1]).unwrap();
    let possible_next_steps: &HashSet<&String> = &possible_next_steps.iter().collect();
    // Get list of already visited rooms
    let visited: HashSet<&String> = cur_path.iter().filter(|s| (***s).to_lowercase() == (***s)).map(|s| *s).collect();
    //println!("visited: {:?}", visited);

    let possible_next_steps: Vec<&String> = possible_next_steps.difference(&visited).map(|s| *s).collect();
    //println!("possible_next_steps: {:?}", possible_next_steps);

    let mut number_of_paths: usize = 0;
    for step in possible_next_steps.iter() {
        let mut new_path = cur_path.clone();
        new_path.push(step);
        number_of_paths += num_paths1(&new_path, cave_map);
    }

    number_of_paths
}

fn num_paths2(cur_path: &Vec<&String>, cave_map: &HashMap<String,HashSet<String>>) -> usize {
    if cur_path[cur_path.len()-1] == "end" {
        // We made it to the end. this is a valid path.
        return 1;
    }
    //println!("cur_path: {:?}", cur_path);
    // Get room list of last room in the path
    let possible_next_steps: &HashSet<String> = cave_map.get(cur_path[cur_path.len()-1]).unwrap();
    let possible_next_steps: &HashSet<&String> = &possible_next_steps.iter().collect();
    // Get list of rooms we can no longer visit. (always start with start)
    let mut cant_revisit: HashSet<&String> = HashSet::from_iter([cur_path[0]]);
    // Determine if we've visited a small cave twise
    let small_caves_visited: Vec<&String> = cur_path.iter().filter(|s| (***s).to_lowercase() == (***s)).map(|s| *s).collect();
    let mut cave_count: HashSet<&String> = HashSet::new();
    let mut visited_small_cave_twice = false;
    for cave in small_caves_visited {
        if !cave_count.contains(cave) {
            cave_count.insert(cave);
        } else {
            visited_small_cave_twice = true;
        }
    }
    if visited_small_cave_twice {
        let small_caves_visited: HashSet<&String> = cur_path.iter().filter(|s| (***s).to_lowercase() == (***s)).map(|s| *s).collect();
        for cave in small_caves_visited.iter().map(|s| *s) {
            cant_revisit.insert(cave);
        }
    }

    let possible_next_steps: Vec<&String> = possible_next_steps.difference(&cant_revisit).map(|s| *s).collect();
    //println!("possible_next_steps: {:?}", possible_next_steps);

    let mut number_of_paths: usize = 0;
    for step in possible_next_steps.iter() {
        let mut new_path = cur_path.clone();
        new_path.push(step);
        number_of_paths += num_paths2(&new_path, cave_map);
    }

    number_of_paths
}

fn main() {
    // Argument Parsing
    let mut verbose = false;
    let mut input = String::new();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Day 12 AOC 2021");
        ap.refer(&mut input)
            .add_option(&["-i", "--input"], Store,
            "Filepath to input data");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue,
            "Add additional reporting");
        ap.parse_args_or_exit();
    }

    let cave_map = process_input(input);
    //println!("{:?}", cave_map);

    let paths = num_paths1(&Vec::<&String>::from_iter([&String::from("start")]),&cave_map);
    println!("Day 12 problem 1: {}", paths);

    let paths = num_paths2(&Vec::<&String>::from_iter([&String::from("start")]),&cave_map);
    println!("Day 12 problem 2: {}", paths);
}
