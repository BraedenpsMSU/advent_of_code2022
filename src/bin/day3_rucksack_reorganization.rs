use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct XmasBag {
    // bag: String,
    bag_contains: u64,
    compartment1_contains: u64,
    compartment2_contains: u64
}

impl XmasBag {
    pub fn ascii_to_xmas_priority(letter: &char) -> u64 {
        // maps 'a' => 0  ... maps 'A' => 26
        // maps 'b' => 1  ... maps 'B' => 27
        // maps 'c' => 2  ... maps 'C' => 28
        // ...                  ...
        // maps 'z' => 25 ... maps 'Z' => 51
        let letter_value: u64 = *letter as u64;
        52
            + (letter_value-1)%0x20 //letter index when alphabet is indexed by 0
            - 26*(letter_value/0x20 -1) //determines if letter is uppercase
    }

    pub fn build_xmas_bag(value: &String) -> XmasBag {
        // iterate over a the string and vectors appropriately.
        let bag_size = value.len() as u64;
        assert_eq!(bag_size % 2, 0);
        // the size of our compartment should be half of our bag
        let compartment_size = bag_size / 2;
        let mut compartment1_value: u64 = 0;
        let mut compartment2_value: u64 = 0;
        let mut bag_value: u64 = 0;
        for (_, c) in value.as_str()[..(compartment_size as usize)].chars().enumerate() {
            bag_value |= 1 << XmasBag::ascii_to_xmas_priority(&c);
            compartment1_value |= 1 << XmasBag::ascii_to_xmas_priority(&c)
        }
        for (_, c) in value.as_str()[(compartment_size as usize)..].chars().enumerate() {
            bag_value |= 1 << XmasBag::ascii_to_xmas_priority(&c);
            compartment2_value |= 1 << XmasBag::ascii_to_xmas_priority(&c)
        }
        assert_eq!(compartment1_value | compartment2_value, bag_value);
        XmasBag {
            bag_contains: bag_value,
            compartment1_contains: compartment1_value,
            compartment2_contains: compartment2_value
        }
    }

    pub fn get_total_priority(xmas_bag: &XmasBag) -> u64 {
        // iterate all values shared between bags
        let mut shared = xmas_bag.compartment1_contains & xmas_bag.compartment2_contains;
        let mut current_priority: u64 = 1;
        let mut accumulated_priority: u64 = 0;
        while shared > 0 {
            accumulated_priority += (1 & shared)*current_priority;
            current_priority += 1;
            shared >>= 1;
        }
        accumulated_priority
    }

    pub fn get_total_priority_from_value(xmas_bag_items: u64) -> u64 {
        // iterate all values shared between bags
        let mut shared = xmas_bag_items;
        let mut current_priority: u64 = 1;
        let mut accumulated_priority: u64 = 0;
        while shared > 0 {
            accumulated_priority += (1 & shared)*current_priority;
            current_priority += 1;
            shared >>= 1;
        }
        accumulated_priority
    }

    // TODO: should be a display or debug function or something
    pub fn show_contents(&self) {
        println!("ZYXWVUTSRQPONMLKJIHGFEDCBAzyxwvutsrqponmlkjihgfedcba");
        println!("{: >52}", format!("{:b}",  self.bag_contains ).as_str());
        println!("{: >52}", format!("{:b}",  self.compartment1_contains ).as_str());
        println!("{: >52}", format!("{:b}",  self.compartment2_contains ).as_str());
    }
}

pub fn get_total_priority_of_bags(file_path: &str) -> u64 {
    let file = File::open(file_path).expect("Cannot open.txt");
    let reader = BufReader::new(file);

    let mut total:u64 = 0;
    for line in reader.lines() {
        let line_info = line.expect("Something went wrong when reading in a line");
        let current_bag = XmasBag::build_xmas_bag(&line_info);
        // current_bag.show_contents();
        total += XmasBag::get_total_priority(&current_bag);
    }
    return total;
}

pub fn get_total_priority_out_of_three_bags(file_path: &str) -> u64 {
    let file = File::open(file_path).expect("Cannot open.txt");
    let reader = BufReader::new(file);
    let mut xmas_bags: Vec<XmasBag> = Vec::new();

    let mut total:u64 = 0;
    for line in reader.lines() {
        let line_info = line.expect("Something went wrong when reading in a line");
        xmas_bags.push( XmasBag::build_xmas_bag(&line_info));
    }
    while !xmas_bags.is_empty() {
        let xmas_bag1 = xmas_bags.pop().expect("There was no bag1");
        let xmas_bag2 = xmas_bags.pop().expect("There was no bag2");
        let xmas_bag3 = xmas_bags.pop().expect("There was no bag3");
        total += XmasBag::get_total_priority_from_value(
            xmas_bag1.bag_contains
                & xmas_bag2.bag_contains
                & xmas_bag3.bag_contains
        )
    }

    return total;
}

pub fn main() {
    let priority: u64 = get_total_priority_of_bags("test_inputs/Day3Part1Example.txt");
    println!("{priority}");
    let priority: u64 = get_total_priority_of_bags("inputs/Day3Part1.txt");
    println!("{priority}");
    let priority: u64 = get_total_priority_out_of_three_bags("test_inputs/Day3Part1Example.txt");
    println!("{priority}");
    let priority: u64 = get_total_priority_out_of_three_bags("inputs/Day3Part1.txt");
    println!("{priority}");
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_ascii_to_character() {
        assert_eq!(XmasBag::ascii_to_xmas_priority(&'a'), 0);
        assert_eq!(XmasBag::ascii_to_xmas_priority(&'z'), 25);
        assert_eq!(XmasBag::ascii_to_xmas_priority(&'A'), 26);
        assert_eq!(XmasBag::ascii_to_xmas_priority(&'Z'), 51);

    }
}