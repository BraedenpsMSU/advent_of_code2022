use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Not;

fn determine_if_not_equal(input: &VecDeque<char>) -> bool {
    let mut values: HashSet<char> = HashSet::new();
    for item in input {values.insert(item.clone());}
    return (values.len() == input.len())
}

fn get_start_of_packet(input: String, _usize: i32) -> i32  {
    let mut pos = _usize.clone();
    let mut current: VecDeque<char> = VecDeque::new();
    let mut input_copy = input.clone();
    for _ in 0.._usize {current.push_back(input_copy.remove(0))}
    loop {
        if determine_if_not_equal(&current) {
            return pos;
        } else { pos +=1; current.pop_front(); current.push_back(input_copy.remove(0))}
    }
}

fn main() {
    let out1 = get_start_of_packet(String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),4);
    println!("{out1}");
    let file = File::open(&String::from("inputs/Day6Part1.txt")).expect("Cannot open.txt");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let input = line.unwrap();
        println!("{}",get_start_of_packet(input.clone(),4));
        println!("{}",get_start_of_packet(input,14))
    }
}