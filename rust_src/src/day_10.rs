extern crate argparse;

use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;
use std::collections::{HashMap,VecDeque};
use std::hash::Hash;
use std::iter::FromIterator;
use argparse::{ArgumentParser, StoreTrue, Store};

// Define readlines function discussed here:
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn process_input<P>(filename: P) -> Vec<String>
where P: AsRef<Path>, {
    let mut results = Vec::new();
    // Reading the file
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                results.push(ip)
            }
        }
    } else {
		panic!("Problem opening file!");
	}
    results
}

#[derive(Debug,PartialEq,Eq,Hash,Clone,Copy)]
enum Block {
    Regular,
    Square,
    Curly,
    Angle,
}

fn get_illegal_char(line: &String) -> Option<Block> {
    let char_map: HashMap<char,Block> = HashMap::from_iter([
        ('(', Block::Regular),
        (')', Block::Regular),
        ('[', Block::Square),
        (']', Block::Square),
        ('{', Block::Curly),
        ('}', Block::Curly),
        ('<', Block::Angle),
        ('>', Block::Angle),
    ]);
    let mut block_stack: VecDeque<Block> = VecDeque::new();
    for ch in line.chars() {
        if ['(', '[', '{', '<'].contains(&ch) {
            let cur_block = char_map.get(&ch).unwrap();
            block_stack.push_front(*cur_block);
        } else if [')', ']', '}', '>' ].contains(&ch) {
            let cur_block = char_map.get(&ch).unwrap();
            let last_block = block_stack.pop_front().unwrap();
            if *cur_block != last_block {
                return Some(*cur_block);
            }
        } else {
            panic!("Encountered unexpected character! {}", ch);
        }
    }
    None
}

fn get_line_completion(line: &String) -> Option<Vec<Block>> {
    let char_map: HashMap<char,Block> = HashMap::from_iter([
        ('(', Block::Regular),
        (')', Block::Regular),
        ('[', Block::Square),
        (']', Block::Square),
        ('{', Block::Curly),
        ('}', Block::Curly),
        ('<', Block::Angle),
        ('>', Block::Angle),
    ]);
    let mut block_stack: VecDeque<Block> = VecDeque::new();
    for ch in line.chars() {
        if ['(', '[', '{', '<'].contains(&ch) {
            let cur_block = char_map.get(&ch).unwrap();
            block_stack.push_front(*cur_block);
        } else if [')', ']', '}', '>' ].contains(&ch) {
            let cur_block = char_map.get(&ch).unwrap();
            let last_block = block_stack.pop_front().unwrap();
            if *cur_block != last_block {
                return None;
            }
        } else {
            panic!("Encountered unexpected character! {}", ch);
        }
    }
    Some(block_stack.iter().copied().collect::<Vec::<Block>>())
}

fn score_line(blocks: &Vec<Block>) -> usize {
    let add_map: HashMap<Block,usize> = HashMap::from_iter([
        (Block::Regular, 1),
        (Block::Square, 2),
        (Block::Curly, 3),
        (Block::Angle, 4)
    ]);

    let mut result = 0;
    for b in blocks {
        result = result*5+add_map.get(&b).unwrap();
    }
    result
}

fn main() {
    // Argument Parsing
    let mut verbose = false;
    let mut input = String::new();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Day 10 AOC 2021");
        ap.refer(&mut input)
            .add_option(&["-i", "--input"], Store,
            "Filepath to input data");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue,
            "Add additional reporting");
        ap.parse_args_or_exit();
    }

    let score_map: HashMap<Block,usize> = HashMap::from_iter([
        (Block::Regular, 3),
        (Block::Square, 57),
        (Block::Curly, 1197),
        (Block::Angle, 25137)
    ]);

    let nav_data = process_input(input);

    let syntax_error_score: usize = nav_data.iter()
        .map(|s| get_illegal_char(s))
        .filter(|r| {
            match r {
                None => false,
                Some(_) => true,
            }
        })
        .map(|r| r.unwrap())
        .map(|r| score_map.get(&r).unwrap()).sum();

    println!("Day 10 problem 1: {}", syntax_error_score);

    let mut line_scores: Vec<usize> = nav_data.iter()
        .map(|s| get_line_completion(s))
        .filter(|r| {
            match r {
                None => false,
                Some(_) => true,
            }
        })
        .map(|r| r.unwrap())
        .map(|v| {
            score_line(&v)
        }).collect::<Vec::<usize>>();

    line_scores.sort();

    let middle_score = line_scores[(line_scores.len()/2)];

    println!("Day 10 problem 2: {}", middle_score);
}
