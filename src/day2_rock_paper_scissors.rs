use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};


pub fn get_score(file_path: &str) -> i32 {
    let file = File::open(file_path).expect("Cannot open.txt");
    let reader = BufReader::new(file);

    // A, X is rock     score 1
    // C, Z is scissors score 3
    // B, Y is paper    score 2
    // Lost = 0, Draw = 3, win 6
    let mut codex: HashMap<&str, i32>  = HashMap::new();
    codex.insert("A X", 1 + 3);
    codex.insert("A Z", 1 + 6);
    codex.insert("A Y", 1 + 0);
    codex.insert("C X", 3 + 0);
    codex.insert("C Z", 3 + 3);
    codex.insert("C Y", 3 + 6);
    codex.insert("B X", 2 + 6);
    codex.insert("B Z", 2 + 0);
    codex.insert("B Y", 2 + 3);

    let mut total:i32 = 0;
    for line in reader.lines() {
        let name = line.expect("Something went wrong when reading in a line");
        total += codex.get(name.as_str()).expect("Failed to get integer");
        let value = codex.get(name.as_str()).expect("Failed to get integer");
        println!("{name}: {value}");
    }

    return total;
}