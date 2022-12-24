use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn find_most_calorie_held_by_elf(file_path: &str) -> i32 {
    let file = File::open(file_path).expect("Cannot open.txt");
    let reader = BufReader::new(file);

    let mut max = 0;
    let mut current_sum = 0;

    for line in reader.lines() {
        let name = line.expect("Something went wrong when reading in a line");
        if name == "" {
            if current_sum >= max {
                max = current_sum;
            }
            current_sum = 0;
        } else {
            let value = name.parse::<i32>().unwrap();
            current_sum += value;
        }
    }
    if current_sum >= max {
        max = current_sum;
    }
    return max;
}

pub fn find_sum_of_3_best_elves(file_path: &str) -> i32 {
    let file = File::open(file_path).expect("Cannot open.txt");
    let reader = BufReader::new(file);

    let mut elves_calories: Vec<i32> = Vec::new();
    let mut current_sum = 0;

    for line in reader.lines() {
        let name = line.expect("Something went wrong when reading in a line");
        if name == "" {
            elves_calories.push(current_sum);
            current_sum = 0;
        } else {
            let value = name.parse::<i32>().unwrap();
            current_sum += value;
        }
    }
    elves_calories.push(current_sum);
    elves_calories.sort_by(|a, b| b.cmp(a));
    let x = (&elves_calories[..3]).iter().sum::<i32>();

    return x;
}

pub fn main() {
    let max_calories: i32 = find_most_calorie_held_by_elf("inputs/Day1Part1.txt");
    println!("{max_calories}");
    let max_calories: i32 = find_sum_of_3_best_elves("inputs/Day1Part1.txt");
    println!("{max_calories}");
}
