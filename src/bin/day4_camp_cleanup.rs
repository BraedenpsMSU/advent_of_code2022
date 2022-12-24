use std::fs::File;
use std::io::{BufRead, BufReader};

struct BagRange {
    lower: u32,
    upper: u32
}

impl BagRange {
    pub fn build_from_string(bag_range: &str) -> BagRange {
        let value: Vec<&str> = bag_range.rsplit('-').collect();
        let lower_str = value.get(1).expect("No first value");
        let upper_str = value.get(0).expect("No second value");
        let lower_value = String::from(*lower_str).parse::<u32>().unwrap();
        let upper_value = String::from(*upper_str).parse::<u32>().unwrap();
        BagRange {
            lower: lower_value,
            upper: upper_value
        }
    }

    pub fn contains(&self, other: &BagRange) -> bool{
        return (self.lower <= other.lower) && (other.upper <= self.upper);
    }

    pub fn overlaps(&self, other: &BagRange) -> bool{
        let check_lower_tail = (self.lower <= other.lower) && (other.lower <= self.upper);
        let check_upper_tail = (self.lower <= other.upper) && (other.upper <= self.upper);
        return check_lower_tail || check_upper_tail;
    }
}

pub fn get_total_containment(file_path: &str) -> u64 {
    let file = File::open(file_path).expect("Cannot open.txt");
    let reader = BufReader::new(file);
    let mut xmas_bags: Vec<BagRange> = Vec::new();

    for line in reader.lines() {
        let line_info = line.expect("Something went wrong when reading in a line");
        let divison: Vec<&str> = line_info.rsplit(',').collect();
        let bag2 = *divison.get(1).expect("No first value");
        let bag1 = *divison.get(0).expect("No second value");
        xmas_bags.push( BagRange::build_from_string(bag1));
        xmas_bags.push( BagRange::build_from_string(bag2));
    }

    let mut total:u64 = 0;
    while !xmas_bags.is_empty() {
        let bag1 = xmas_bags.pop().expect("Failed to get 1st bag");
        let bag2 = xmas_bags.pop().expect("Failed to get 2nd bag");
        if bag2.contains(&bag1) || bag1.contains(&bag2) {
            total += 1;
        }
    }
    return total;
}

pub fn get_total_overlap(file_path: &str) -> u64 {
    let file = File::open(file_path).expect("Cannot open.txt");
    let reader = BufReader::new(file);
    let mut bags: Vec<BagRange> = Vec::new();

    for line in reader.lines() {
        let line_info = line.expect("Something went wrong when reading in a line");
        let divison: Vec<&str> = line_info.rsplit(',').collect();
        let bag2 = *divison.get(1).expect("No first value");
        let bag1 = *divison.get(0).expect("No second value");
        bags.push( BagRange::build_from_string(bag1));
        bags.push( BagRange::build_from_string(bag2));
    }

    let mut total:u64 = 0;
    while !bags.is_empty() {
        let bag1 = bags.pop().expect("Failed to get 1st bag");
        let bag2 = bags.pop().expect("Failed to get 2nd bag");
        if bag2.overlaps(&bag1) || bag1.overlaps(&bag2) {
            total += 1;
        }
    }
    return total;
}

pub fn main() {
    let number_of_contains = get_total_containment("test_inputs/Day4Part1Example.txt");
    println!("{number_of_contains}");
    let number_of_contains = get_total_containment("inputs/Day4Part1.txt");
    println!("{number_of_contains}");
    let number_of_overlap = get_total_overlap("test_inputs/Day4Part1Example.txt");
    println!("{number_of_overlap}");
    let number_of_overlap = get_total_overlap("inputs/Day4Part1.txt");
    println!("{number_of_overlap}");
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_ascii_to_character() {
        assert_eq!(BagRange::build_from_string("1-20").lower, 1);
        assert_eq!(BagRange::build_from_string("1-20").upper, 20);
    }
}