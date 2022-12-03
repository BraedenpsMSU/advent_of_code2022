use std::fs::File;
use std::io::{BufRead, BufReader};


pub fn find_most_calorie(file_path: &str) -> i32 {
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
