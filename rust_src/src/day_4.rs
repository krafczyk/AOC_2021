extern crate argparse;

use std::io::{self, BufRead};
use std::fs::File;
use std::str::FromStr;
use std::fmt::Display;
use std::path::Path;
use std::collections::HashMap;
//use ndarray::prelude::*;
//use ndarray::Array;
use argparse::{ArgumentParser, StoreTrue, Store};

// Define readlines function discussed here:
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn print_type_of<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn process_input<P, T>(filename: P) -> (Vec<T>, Vec<Vec<Vec<T>>>)
where
	P: AsRef<Path>,
	T: FromStr+Display+std::fmt::Debug,
	<T as FromStr>::Err: std::fmt::Debug,
{
    let mut numbers: Vec<T> = Vec::new();
    let mut boards: Vec<Vec<Vec<T>>> = Vec::new();
    // Reading the file
    if let Ok(lines) = read_lines(filename) {
        // I'm not sure why I need to do this, but I've converted the lines iterator to
        // a mutable iterator using the map function.
        let mut line_it = lines.map(|x| x);
        // Extract chosen numbers from first line of file
        if let Some(Ok(line)) = line_it.next() {
            for num_str in line.split(',') {
                if let Ok(num) = num_str.parse::<T>() {
					numbers.push(num);
				}
			}
		} else {
			println!("Failure Extracting selected numbers!");
        }

		// Now we begin extracting the boards

		loop {
        	if let Some(Ok(_)) = line_it.next() {
			} else {
				// We should break out now.
				break
			}
            // Initialize new board
			let mut board: Vec<Vec<T>> = Vec::new();

			for _ in 0..5 {
				if let Some(Ok(line)) = line_it.next() {
                    let line_tokens = line.split_whitespace();
                    let line_numbers = line_tokens.map(|x| x.parse::<T>());
                    let line_numbers = line_numbers.filter(|x| if let Ok(_) = x { return true } else { return false} );
                    let line_numbers: Vec<T> = line_numbers.map(|x| x.unwrap() ).collect();
                    board.push(line_numbers);
				} else {
					panic!("We encountered early end to data!");
				}
			}

			// Added new board to list of boards
			boards.push(board);
		}

		//println!("There were {} boards!", boards.len());
    }
    return (numbers, boards)
}

fn show_sorted_pairs<K,V>(map: &HashMap<K,V>)
where
	K: Copy+Ord+std::fmt::Debug,
	V: Copy+std::fmt::Debug,
{
	let mut pairs = map.iter().map(|(&num, &count)| (num, count)).collect::<Vec::<(K,V)>>();
    pairs.sort_by_key(|(num,_)| *num);
    println!("{:?}", pairs);
}

fn build_bingo_paths<T>(board: &Vec<Vec<T>>) -> Vec<Vec<&T>> {
	// Diagnoal paths
    let mut paths: Vec<Vec<&T>> = Vec::new();

	//let mut diag: Vec<&T> = Vec::new();
	//let mut anti_diag: Vec<&T> = Vec::new();
    //for i in 0..5 {
	//	diag.push(&(board[i][i]));
	//	anti_diag.push(&(board[4-i][i]));
	//}
    //paths.push(diag);
	//paths.push(anti_diag);

	for i in 0..5 {
		let mut horiz: Vec<&T> = Vec::new();
		for j in 0..5 {
			horiz.push(&(board[i][j]));
		}
		paths.push(horiz);
	}

	for i in 0..5 {
		let mut vert: Vec<&T> = Vec::new();
		for j in 0..5 {
			vert.push(&(board[j][i]));
		}
		paths.push(vert);
	}
	paths
}

fn check_board_for_bingo<T>(numbers: &[T], board_paths: &Vec<Vec<&T>>) -> bool
where
	T: PartialEq
{
    let isin_numbers = |num: &&T| -> bool {
        numbers.contains(*num)
    };
	board_paths.iter().any(|path| {
		path.iter().all(isin_numbers)
	})
}


fn main() {
    // Argument Parsing
    let mut verbose = false;
    let mut input = String::new();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Day 4 AOC 2021");
        ap.refer(&mut input)
            .add_option(&["-i", "--input"], Store,
            "Filepath to input data");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue,
            "Add additional reporting");
        ap.parse_args_or_exit();
    }

    // Read chosen numbers and boards from input data.
    let (numbers, boards) = process_input::<_,u32>(input);

    // Build board paths
    let bingo_paths: Vec<Vec<Vec<&u32>>> = boards.iter().map(build_bingo_paths).collect();

    //for i in 0..numbers.len() {
    for i in 0..numbers.len() {
        let nums = &numbers[0..i+1];
        let board_results: Vec<bool> =
            bingo_paths.iter()
                       .map(|b| check_board_for_bingo(nums, b))
                       .collect();
        let res: Vec<usize> =
            board_results.into_iter()
                         .enumerate()
                         .filter(|(_, b)| *b)
                         .map(|(i,_)| i).collect();
        if res.len() > 0 {
            let winner_idx = res[0];
            let last_num = nums[nums.len()-1];
            let board = &boards[winner_idx];
            let remaining_num_sum: u32 = board.iter()
                                         .flatten()
                                         .filter(|num| !nums.contains(num))
                                         .sum();
            println!("Day 4 problem 1: {}", remaining_num_sum*last_num);
            break;
        }
    }

	//println!("bingo paths");
	//for path in bingo_paths {
	//	println!("{:?}", path);
	//}

	//let func = |b: &Vec<Vec<_>>| {
	//	print_type_of(b);
	//	let new_b = array!(*b);
	//	print_type_of(&new_b);
	//	println!("{}", &new_b.ndim());
	//	println!("{:?}", &new_b.shape());
	//};

    //boards.iter().map(func).collect::<Vec::<()>>();
}
