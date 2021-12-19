extern crate argparse;
extern crate regex;

use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;
use std::ops::{Add,Sub};
use std::fmt::Debug;
use argparse::{ArgumentParser, StoreTrue, Store};
use regex::Regex;

// Define readlines function discussed here:
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn process_input<P>(filename: P) -> (String, Vec<(String,String)>)
where
    P: AsRef<Path>,
{
    // Reading the file
    if let Ok(mut lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        let mut rules: Vec<(String,String)> = Vec::new();
        let init_formula = lines.next().unwrap().unwrap();
        lines.next();
        let rule_re = Regex::new(r"([A-Z]*) -> ([A-Z]*)").unwrap();
        for line in lines {
            if let Ok(ip) = line {
                match rule_re.captures(&ip) {
                    Some(cap) => {
                        let left = cap.get(1).unwrap().as_str().to_string();
                        let right = cap.get(2).unwrap().as_str().to_string();
                        rules.push((left, right));
                    },
                    None => {
                    },
                }
            }
        }
        (init_formula, rules)
    } else {
		panic!("Problem opening file!");
	}
}

fn build_rule_table(in_rules: &Vec<(String,String)>) -> HashMap<String,(String,String)>
{
    let mut rules: HashMap<String,(String,String)> = HashMap::new();
    for (left, right) in in_rules {
        let left = left.chars().collect::<Vec::<char>>();
        let right = right.chars().collect::<Vec::<char>>();
        let left_pat: String = [left[0],right[0]].iter().collect();
        let right_pat: String = [right[0],left[1]].iter().collect();
        rules.insert(left.iter().collect(), (left_pat, right_pat));
    }
    rules
}

fn formula_to_tokens<T>(formula: &String) -> HashMap<String, T>
where
    T: Add+Add<Output=T>,
    T: num::NumCast,
    T: Copy,
{
    let mut tokens: HashMap<String, T> = HashMap::new();
    let formula_chars: Vec<char> = formula.chars().collect();
    for i in 0..formula_chars.len()-1 {
        let token: String = [formula_chars[i],formula_chars[i+1]].iter().collect();
        let val = tokens.entry(token).or_insert(num::cast::<_,T>(0).unwrap());
        *val = *val + num::cast::<_,T>(1).unwrap();
    }
    tokens
}

fn expand_formula<T>(formula: &HashMap<String,T>, rules: &HashMap<String,(String,String)>) -> HashMap<String,T>
where
    T: Add+Add<Output=T>,
    T: num::NumCast,
    T: Copy,
    T: Debug,
{
    //println!("======= new expand formula: {:?}", formula);
    let mut new_formula: HashMap<String,T> = HashMap::new();
    for (key, val) in formula.iter() {
        //println!("Transforming key: {}", key);
        let new_keys = rules.get(key).unwrap();
        //println!("new keys: {:?}", new_keys);

        let old_val = new_formula.entry(new_keys.0.clone()).or_insert(num::cast::<_,T>(0).unwrap());
        *old_val = *old_val+*val;
        let old_val = new_formula.entry(new_keys.1.clone()).or_insert(num::cast::<_,T>(0).unwrap());
        *old_val = *old_val+*val;
        //println!("Updated new formula: {:?}", new_formula);
    }
    //println!("new formula: {:?}", new_formula);
    new_formula
}

fn char_count<T>(formula_tokens: &HashMap<String,T>, first_char: char) -> HashMap<char,T>
where
    T: Add+Add<Output=T>,
    T: num::NumCast,
    T: Copy,
{
    let mut char_count: HashMap<char,T> = HashMap::new();
    let old_val = char_count.entry(first_char).or_insert(num::cast::<_,T>(0).unwrap());
    *old_val = *old_val+num::cast::<_,T>(1).unwrap();
    for (token, num) in formula_tokens.iter() {
       let the_char = token.chars().nth(1).unwrap();
       let old_val = char_count.entry(the_char).or_insert(num::cast::<_,T>(0).unwrap());
       *old_val = *old_val+(*num);
    }
    char_count
}

fn compute_diff<T>(char_count: &HashMap<char,T>) -> T
where
    T: Add+Add<Output=T>,
    T: Sub+Sub<Output=T>,
    T: num::NumCast,
    T: Copy,
    T: Ord,
{
    let min_v = char_count.values().min().unwrap();
    let max_v = char_count.values().max().unwrap();
    return *max_v-*min_v;
}

fn main() {
    // Argument Parsing
    let mut verbose = false;
    let mut input = String::new();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Day 14 AOC 2021");
        ap.refer(&mut input)
            .add_option(&["-i", "--input"], Store,
            "Filepath to input data");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue,
            "Add additional reporting");
        ap.parse_args_or_exit();
    }

    let (init_formula, rules) = process_input(input);
    let first_char = init_formula.chars().nth(0).unwrap();

    let rules = build_rule_table(&rules);
    let mut formula_tokens = formula_to_tokens::<usize>(&init_formula);

    let mut counts: Vec<usize> = Vec::new();
    counts.push(compute_diff::<usize>(&char_count::<usize>(&formula_tokens, first_char)));

    let num_steps: usize = 40;
    for _ in 0..num_steps {
        formula_tokens = expand_formula::<usize>(&formula_tokens, &rules);
        counts.push(compute_diff::<usize>(&char_count::<usize>(&formula_tokens, first_char)));
    }

    println!("Day 14 problem 1: {}", counts[10]);
    println!("Day 14 problem 2: {}", counts[40]);
}
