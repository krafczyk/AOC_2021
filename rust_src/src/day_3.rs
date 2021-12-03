extern crate argparse;

use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;
use std::result::Result;
use std::collections::HashMap;
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
    // Reading the file
    let mut result = Vec::<String>::new();
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                result.push(ip);
            }
        }
    }   
    return result;
}

fn validate_input(input: &Vec<String>) -> Result<(),()> {
    let mut length: usize = 0;
    let mut is_first = true;
    for line in input {
        if is_first {
            length = line.len();
            is_first = false;
        } else {
            if line.len() != length {
                return Result::Err(());
            }
        }
    }
    return Result::Ok(());
}

fn problem_1(input: &Vec<String>) {
    let str_length: usize = input[0].len();
    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();
    let mut stats = HashMap::<usize,HashMap::<char,usize>>::new();
    for line in input {
        for i in 0..str_length {
            let the_char = line.chars().nth(i).unwrap();
            loop {
                if let Some(map) = stats.get_mut(&i) {
                    if let Some(count) = map.get_mut(&the_char) {
                        *count += 1;
                        break;
                    } else {
                        map.insert(the_char, 1);
                        break;
                    }
                } else {
                    stats.insert(i, HashMap::<char,usize>::new());
                }
            }
        }
    }
    for i in 0..str_length {
        let zeros = stats.get(&i).unwrap().get(&'0').unwrap();
        let ones = stats.get(&i).unwrap().get(&'1').unwrap();
        if zeros > ones {
            gamma_rate.push('0');
            epsilon_rate.push('1');
        } else if ones > zeros {
            gamma_rate.push('1');
            epsilon_rate.push('0');
        } else {
            println!("Unexpected line! equal number of ones and zeros!");
        }
    }
    let gamma_rate = usize::from_str_radix(&gamma_rate, 2).unwrap();
    let epsilon_rate = usize::from_str_radix(&epsilon_rate, 2).unwrap();
    let power_rate = gamma_rate*epsilon_rate;
    println!("Day 3 problem 1: {}", power_rate);
}

fn find_o2gen_rating(input: &Vec<String>, idx: usize) -> usize {
    if input.len() == 1 { 
        return usize::from_str_radix(&input[0], 2).unwrap();
    } else {
        let mut num_zeros: usize = 0;
        let mut num_ones: usize = 0;
        // Count number of ones/zeros in the designated column
        for input_num in input {
            match input_num.chars().nth(idx) {
                Some('0') => num_zeros += 1,
                Some('1') => num_ones += 1,
                Some(_) => println!("Unexpected character!"),
                _ => println!("Some error was encountered!")
            }
        }
        let mut new_list = Vec::<String>::new();
        if num_zeros > num_ones {
            for input_num in input {
                if let Some('0') = input_num.chars().nth(idx) {
                    new_list.push(input_num.clone());
                }
            }
        } else {
            for input_num in input {
                if let Some('1') = input_num.chars().nth(idx) {
                    new_list.push(input_num.clone());
                }
            }
        }
        return find_o2gen_rating(&mut new_list, idx+1);
    }
}

fn find_co2_rating(input: &Vec<String>, idx: usize) -> usize {
    if input.len() == 1 { 
        return usize::from_str_radix(&input[0], 2).unwrap();
    } else {
        let mut num_zeros: usize = 0;
        let mut num_ones: usize = 0;
        // Count number of ones/zeros in the designated column
        for input_num in input {
            match input_num.chars().nth(idx) {
                Some('0') => num_zeros += 1,
                Some('1') => num_ones += 1,
                Some(_) => println!("Unexpected character!"),
                _ => println!("Some error was encountered!")
            }
        }
        let mut new_list = Vec::<String>::new();
        if num_zeros > num_ones {
            for input_num in input {
                if let Some('1') = input_num.chars().nth(idx) {
                    new_list.push(input_num.clone());
                }
            }
        } else {
            for input_num in input {
                if let Some('0') = input_num.chars().nth(idx) {
                    new_list.push(input_num.clone());
                }
            }
        }
        return find_co2_rating(&mut new_list, idx+1);
    }
}

fn problem_2(input: &Vec<String>) {
    let o2gen_rating = find_o2gen_rating(input, 0);
    let co2_rating = find_co2_rating(input, 0);
    let lifesupport_rating = o2gen_rating*co2_rating;
    println!("Day 3 problem 2: {}", lifesupport_rating);
}

fn main() {
    // Argument Parsing
    let mut verbose = false;
    let mut input = String::new();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Day 3 AOC 2021");
        ap.refer(&mut input)
            .add_option(&["-i", "--input"], Store,
            "Filepath to input data");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue,
            "Add additional reporting");
        ap.parse_args_or_exit();
    }

    let input = process_input(input);
    if let Ok(_) = validate_input(&input) {
        problem_1(&input);
        problem_2(&input);
    } else {
        println!("There was a problem validating input!");
    }
}
