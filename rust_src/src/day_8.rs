extern crate argparse;

use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;
use std::collections::{HashSet,HashMap};
use std::convert::TryInto;
use std::iter::FromIterator;
use argparse::{ArgumentParser, StoreTrue, Store};

// Define readlines function discussed here:
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

//fn print_type_of<T>(_: &T) {
//    println!("{}", std::any::type_name::<T>())
//}

fn get_str_from_charset(set: &HashSet<char>) -> String {
    let mut char_vec: Vec<&char> = set.iter().collect::<Vec::<&char>>();
    char_vec.sort();
    char_vec.iter()
            .map(|c| c.to_string())
            .collect::<Vec::<String>>().join("")
}

fn build_charset(string: &str) -> HashSet<char> {
    let mut set = HashSet::new();
    for ch in string.chars() {
        set.insert(ch);
    }
    set
}

fn process_input<P>(filename: P) -> Vec<(Vec<HashSet<char>>,Vec<HashSet<char>>)>
where P: AsRef<Path>, {
    let mut result = Vec::new();
    // Reading the file
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let mut unique_patterns: Vec<HashSet<char>> = Vec::new();
                let mut nums: Vec<HashSet<char>> = Vec::new();
                let components: Vec<&str> = ip.split('|').map(|s| s.trim()).collect();
                if components.len() != 2 {
                    panic!("Found incorrectly formatted line!");
                }
                let unique_patterns_strs: Vec<&str> = components[0].split_whitespace().collect();
                let num_strs: Vec<&str> = components[1].split_whitespace().collect();
                for string in unique_patterns_strs {
                    unique_patterns.push(build_charset(string));
                }
                for string in num_strs {
                    nums.push(build_charset(string));
                }
                result.push((unique_patterns, nums));
            }
        }
    } else {
		panic!("Problem opening file!");
	}
    return result;
}

fn build_true_num_seg_map() -> HashMap<i32,HashSet<char>> {
    HashMap::from_iter([
        (0, HashSet::from_iter(['a', 'b', 'c', 'e', 'f', 'g'])),
        (1, HashSet::from_iter(['c', 'f'])),
        (2, HashSet::from_iter(['a', 'c', 'd', 'e', 'g'])),
        (3, HashSet::from_iter(['a', 'c', 'd', 'f', 'g'])),
        (4, HashSet::from_iter(['b', 'c', 'd', 'f'])),
        (5, HashSet::from_iter(['a', 'b', 'd', 'f', 'g'])),
        (6, HashSet::from_iter(['a', 'b', 'd', 'e', 'f', 'g'])),
        (7, HashSet::from_iter(['a', 'c', 'f'])),
        (8, HashSet::from_iter(['a', 'b', 'c', 'd', 'e', 'f', 'g'])),
        (9, HashSet::from_iter(['a', 'b', 'c', 'd', 'f', 'g']))])
}

fn remove_elements_get_vec<'a>(set: &'a HashSet<char>, remove_set: &Vec<&char>) -> Vec<char> {
    let mut set_temp = set.clone();
    for elem in remove_set {
        set_temp.remove(*elem);
    }
    return set_temp.into_iter().collect::<Vec::<char>>();
}

fn decode(code: &(Vec<HashSet<char>>, Vec<HashSet<char>>), true_num_seg_map: &HashMap<i32, HashSet<char>>) -> String {
    // Final code map. Directly maps sets to code numbers
    let mut code_map: HashMap<String,i32> = HashMap::new();
    let mut map_code: HashMap<i32,&HashSet<char>> = HashMap::new();
    // Map between code segments and real segments
    let mut seg_map: HashMap<char,char> = HashMap::new();
    // Categorize unique patterns by number of segments
    let mut num_seg_map: HashMap<i32,Vec<&HashSet<char>>> = HashMap::new();

    // Categorize patterns by number of segments
    for pattern in &(code.0) {
        let num_segs: i32 = pattern.len().try_into().unwrap();
        let map_vec = num_seg_map.entry(num_segs).or_insert(Vec::<&HashSet::<char>>::new());
        map_vec.push(&pattern);
    }

    // Begin to decode.
    // Build intersection patterns
    let seg5_vec = num_seg_map.get(&5).unwrap();
    let mut seg5_pat = seg5_vec[0].clone();
    for pat in seg5_vec {
        seg5_pat = HashSet::from_iter(seg5_pat.intersection(pat).map(|c| *c));
    }

    let seg6_vec = num_seg_map.get(&6).unwrap();
    let mut seg6_pat = seg6_vec[0].clone();
    for pat in seg6_vec {
        seg6_pat = HashSet::from_iter(seg6_pat.intersection(pat).map(|c| *c));
    }

    // Store known unique patterns
    code_map.insert(get_str_from_charset(num_seg_map.get(&2).unwrap()[0]), 1);
    map_code.insert(1, num_seg_map.get(&2).unwrap()[0]);
    code_map.insert(get_str_from_charset(num_seg_map.get(&3).unwrap()[0]), 7);
    map_code.insert(7, num_seg_map.get(&3).unwrap()[0]);
    code_map.insert(get_str_from_charset(num_seg_map.get(&4).unwrap()[0]), 4);
    map_code.insert(4, num_seg_map.get(&4).unwrap()[0]);
    code_map.insert(get_str_from_charset(num_seg_map.get(&7).unwrap()[0]), 8);
    map_code.insert(8, num_seg_map.get(&7).unwrap()[0]);

    let a_seg: char = *map_code.get(&7).unwrap().intersection(&seg5_pat).collect::<Vec::<&char>>()[0];
    seg_map.insert('a', a_seg);

    let d_seg: char = *map_code.get(&4).unwrap().intersection(&seg5_pat).collect::<Vec::<&char>>()[0];
    seg_map.insert('d', d_seg);
    
    let g_seg: char = remove_elements_get_vec(&seg5_pat, &Vec::<&char>::from([&a_seg, &d_seg]))[0];
    seg_map.insert('g', g_seg);

    let f_seg: char = *seg6_pat.intersection(map_code.get(&1).unwrap()).collect::<Vec::<&char>>()[0];
    seg_map.insert('f', f_seg);

    let c_seg: char = remove_elements_get_vec(map_code.get(&1).unwrap(), &Vec::<&char>::from([&f_seg]))[0];
    seg_map.insert('c', c_seg);

    let b_seg: char = remove_elements_get_vec(map_code.get(&4).unwrap(), &Vec::<&char>::from([&c_seg, &f_seg, &d_seg]))[0];
    seg_map.insert('b', b_seg);

    let e_seg: char = remove_elements_get_vec(map_code.get(&8).unwrap(), &Vec::<&char>::from([&a_seg, &d_seg, &g_seg, &f_seg, &c_seg, &b_seg]))[0];
    seg_map.insert('e', e_seg);

    // Fill out code map
    for (num, true_seg_set) in true_num_seg_map {
        let mut coded_seg: HashSet<char> = HashSet::new();
        for seg in true_seg_set {
            coded_seg.insert(*seg_map.get(seg).unwrap());
        }
      code_map.insert(get_str_from_charset(&coded_seg), *num);
    }

    // Decode message
    return code.1.iter().map(|num| {
        let num_val = code_map.get(&get_str_from_charset(num)).unwrap();
        num_val.to_string()
    }).collect::<Vec::<String>>().join("");
}

fn main() {
    // Argument Parsing
    let mut verbose = false;
    let mut input = String::new();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Day 8 AOC 2021");
        ap.refer(&mut input)
            .add_option(&["-i", "--input"], Store,
            "Filepath to input data");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue,
            "Add additional reporting");
        ap.parse_args_or_exit();
    }

    let true_num_seg_map = build_true_num_seg_map();

    let codes = process_input(input);

    let messages: Vec<String> = codes.iter().map(|c| {
        decode(c, &true_num_seg_map)
    }).collect();

    let mut num_easy = 0;
    let mut num = 0;
    for msg in messages {
        num_easy += msg.chars().filter(|c| ['1', '4', '7', '8'].contains(c)).collect::<Vec::<char>>().len();
        num += msg.parse::<i32>().unwrap();
    }
    println!("Day 8 problem 1: {}", num_easy);
    println!("Day 8 problem 2: {}", num);
}
