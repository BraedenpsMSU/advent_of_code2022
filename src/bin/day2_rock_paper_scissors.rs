use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

// part 1

pub fn make_score_codex() -> HashMap<&'static str, i32> {
    let mut codex: HashMap<&str, i32> = HashMap::new();
    codex.insert("A X", 1 + 3);
    codex.insert("A Z", 3 + 0);
    codex.insert("A Y", 2 + 6);
    codex.insert("C X", 1 + 6);
    codex.insert("C Z", 3 + 3);
    codex.insert("C Y", 2 + 0);
    codex.insert("B X", 1 + 0);
    codex.insert("B Z", 3 + 6);
    codex.insert("B Y", 2 + 3);
    return codex
}


pub fn get_score_when_told_next_move(file_path: &str) -> i32 {
    let file = File::open(file_path).expect("Cannot open.txt");
    let reader = BufReader::new(file);

    // A, X is rock     score 1
    // C, Z is scissors score 3
    // B, Y is paper    score 2
    // Lost = 0, Draw = 3, win 6
    let codex: HashMap<&str, i32> = make_score_codex();
    let mut total:i32 = 0;
    for line in reader.lines() {
        let name = line.expect("Something went wrong when reading in a line");
        total += codex.get(name.as_str()).expect("Failed to get integer");
    }
    return total;
}

// part 2

pub fn action_to_number(_move: &str) -> i32 {
    // coverts a given move to a number from 0 to 2 inclusive.
    // This is done is in such away that [f(move) + 1 mod 3] is always the number of move
    // that beats the move given.
    match _move {
        "X" => 0,
        "Y" => 1,
        "Z" => 2,
        "A" => 0,
        "B" => 1,
        "C" => 2,
        _ => {println!("Action_to_number:: input is invalid {_move:#?}"); -100}
    }
}

pub fn number_to_my_action(action_value: i32) -> String {
    // translate the a number back to the move you would make.
    match action_value {
        0 => String::from("X"),
        1 => String::from("Y"),
        2 => String::from("Z"),
        _ => {println!("Number_to-my_action:: Invalid input {action_value:#?}"); String::from("---")}
    }
}

// state Y -> I need to draw says elf.
// state X -> I need to lose says elf.
// state Z -> I need to win says elf.
// Note as rock paper scissors can be drawn as cycle depicting who wins, e.g.
//      rock -> paper -> scissors -> rock indicate paper beats rock and so on.
//      we many find what move we need to play by moving either 1, 2, or 0
//      spots in the cycle.
//      This can be done using the integer modulo 3.
// That being said we will make a map to store how to move:
pub fn translation_number(my_move: &str) ->  i32 {
    match my_move {
        //      X = "lose" = don't move state = 0
        "X" => 2, // X is rock
        //      Y = "draw" = move back 1 (congruent to -1 mod 3 = 2 mod 3) = 2
        "Y" => 0, // Y is paper
        //      Z = "win" = move forward 1 = 1
        "Z" => 1, // Z is scissors
        // otherwise print something,
        _ => {println!("translation_number:: Invalid move string {my_move:#?}"); -100}
    }
}

pub fn adapt_outcome_to_move(game_info: &str) -> String {
    // converts a string the which tells you which outcome of a rock paper, scissors game
    // you need to one which tells you which move to make
        let my_move: String = number_to_my_action((
        action_to_number(&game_info[..1])  +
            translation_number(&game_info[2..])
            ) % 3
    );
    let mut prefix = String::from(game_info);
    prefix.remove(2);
    prefix.push_str(my_move.as_str());
    println!("{game_info} => {prefix}");
    return String::from(prefix);
}

pub fn get_score_when_told_outcome(file_path: &str) -> i32 {
    let file = File::open(file_path).expect("Cannot open.txt");
    let reader = BufReader::new(file);

    let mut total:i32 = 0;
    let get_new_score = make_score_codex();
    for line in reader.lines() {
        let line_info = line.expect("Something went wrong when reading in a line");
        let convert_line_info = adapt_outcome_to_move(line_info.as_str());
        total += get_new_score.get(convert_line_info.as_str())
            .expect(format!("Invalid move string provided {}", convert_line_info.as_str())
                .as_str())
    }
    return total;
}

pub fn main() {
    // day 2
    let score_total: i32 = get_score_when_told_next_move("test_inputs/Day2Part1Example.txt");
    println!("{score_total}");
    let score_total: i32 = get_score_when_told_next_move("inputs/Day2Part1.txt");
    println!("{score_total}");
    let score_total: i32 = get_score_when_told_outcome("test_inputs/Day2Part1Example.txt");
    println!("{score_total}");
    let score_total: i32 = get_score_when_told_outcome("inputs/Day2Part1.txt");
    println!("{score_total}");
    let score_total: i32 = get_score_when_told_outcome("test_inputs/Day2Part1Example.txt");
    println!("{score_total}");
}