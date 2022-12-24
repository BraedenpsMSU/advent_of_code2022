use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub trait AoCLineRepObject {
    fn build_from_line(string: String) -> Self;
}

//fn largest<T>(list: &[T]) -> &T {
pub fn build_vec_deque_from_file<T: AoCLineRepObject>(file_path: &String) -> Vec<T> {
    let file = File::open(file_path).expect("Cannot open input file");
    let reader = BufReader::new(file);

    let mut outp: Vec<T> = Vec::new();
    for line in reader.lines() {
        let line_data = line.expect("Something went wrong when reading in a line");
        outp.push(T::build_from_line(line_data.clone()));
    }
    outp
}

pub fn build_from_string_vec_deque<T: AoCLineRepObject>(string_vector: VecDeque<String>) -> VecDeque<T> {
    string_vector.into_iter().map(|item| T::build_from_line(item)).collect()
}

pub trait AoCMultiLineRepObject {
    fn build_from_string_vec_deque(string_vector: VecDeque<String>) -> Self;
}

pub fn build_from_file<T: AoCMultiLineRepObject>(file_path: &String) -> T {
    let file = File::open(file_path).expect("Cannot open input file");
    let reader = BufReader::new(file);

    let mut string_vector: VecDeque<String> = VecDeque::new();
    for line in reader.lines() {
        let line_data = line.expect("Something went wrong when reading in a line");
        string_vector.push_back(line_data);
    }
    T::build_from_string_vec_deque(string_vector)
}
