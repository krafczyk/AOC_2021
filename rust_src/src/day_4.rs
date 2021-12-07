extern crate argparse;

use std::io::{self, BufRead};
use std::fs::File;
use std::str::FromStr;
use std::fmt::Display;
use std::path::Path;
use argparse::{ArgumentParser, StoreTrue, Store};

// Define readlines function discussed here:
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

//fn print_type_of<T>(_: T) {
//    println!("{}", std::any::type_name::<T>());
//}

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

//fn show_sorted_pairs<K,V>(map: &HashMap<K,V>)
//where
//	K: Copy+Ord+std::fmt::Debug,
//	V: Copy+std::fmt::Debug,
//{
//	let mut pairs = map.iter().map(|(&num, &count)| (num, count)).collect::<Vec::<(K,V)>>();
//    pairs.sort_by_key(|(num,_)| *num);
//    println!("{:?}", pairs);
//}

fn build_bingo_paths<T>(board: &Vec<Vec<T>>) -> Vec<Vec<&T>> {
	// Diagnoal paths
    let mut paths: Vec<Vec<&T>> = Vec::new();

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

fn solve_problem<'a, T>(numbers: &'a Vec<T>, boards: &'a Vec<Vec<Vec<T>>>)
where
    T: PartialEq+Copy+std::iter::Sum<&'a T>+std::fmt::Debug+std::ops::Mul<Output=T>+std::fmt::Display
{
    // Build board paths
    let bingo_paths: Vec<Vec<Vec<&T>>> = boards.iter().map(build_bingo_paths).collect();

    let mut winner_idxs: Vec<usize> = Vec::new();
    let mut winner_nums: Vec<T> = Vec::new();
    let mut winner_rem_sums: Vec<T> = Vec::new();
    for i in 0..numbers.len() {
        let nums = &numbers[0..i+1];
        let board_results: Vec<bool> =
            bingo_paths.iter()
                       .map(|b| check_board_for_bingo(nums, b))
                       .collect();
        let winners: Vec<usize> =
            board_results.into_iter()
                         .enumerate()
                         .filter(|(_, b)| *b)
                         .map(|(i,_)| i).collect();
        // Remove winners that have already been found
        let winners: Vec<usize> = winners.iter().copied().filter(|num| !winner_idxs.contains(num)).collect();
        if winners.len() > 0 {
            let last_num = nums[nums.len()-1];
            for win_idx in winners {
                // Add the new winner to the list
                winner_nums.push(last_num);
                winner_idxs.push(win_idx);
                // Compute the remaining sum for the board
                let board = &boards[win_idx];
                let remaining_num_sum: T = board.iter()
                                                .flatten()
                                                .filter(|num| !nums.contains(num))
                                                .sum();
                // Add the new remaining sum to the list.
                winner_rem_sums.push(remaining_num_sum);
            }
        }
    }

    let len = winner_nums.len();

    println!("Day 4 problem 1: {}", winner_nums[0]*winner_rem_sums[0]);
    println!("Day 4 problem 2: {}", winner_nums[len-1]*winner_rem_sums[len-1]);
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

    // Solve the day's problem
    solve_problem::<u32>(&numbers, &boards);
}
