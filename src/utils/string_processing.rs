use std::collections::VecDeque;

pub fn pull_all_words(input_text: String) -> VecDeque<String> {
    return input_text.split_ascii_whitespace()
        .into_iter()
        .map(|x| String::from(x))
        .collect();
}

pub fn pull_all_numbers(input_text: String) -> VecDeque<i64> {
    return input_text.split_ascii_whitespace()
        .into_iter()
        .map(|x| String::from(x))
        .filter(|x| x.parse::<i64>().is_ok())
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

pub fn chunk(input_text: String, _size: u64, spacing: u64, offset: u64) -> VecDeque<String>{
    let mut input_text_copy = input_text.clone();
    let mut output: VecDeque<String> = VecDeque::new();
    for _ in 0..offset {
        input_text_copy.remove(0);
    }
    let mut current_word = String::from("");
    while !input_text_copy.is_empty() {
        for _ in 0.._size {
            current_word.push(input_text_copy.remove(0));
        }
        output.push_back(current_word.clone());
        current_word = String::from("");
        for _ in 0..spacing {
            if input_text_copy.is_empty() {break;}
            input_text_copy.remove(0);
        }
    }
    output
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_word_split() {
        let outp = pull_all_words(String::from("  hello  \t \n word 1234 "));
        assert_eq!(outp.front().unwrap(), "hello")
    }

    #[test]
    fn test_number_split() {
        let outp = pull_all_numbers(String::from("  hello 111234 \t \n word 1234 "));
        assert_eq!(*outp.front().unwrap(), 111234)
    }

    #[test]
    fn test_chunk_split() {
        let outp = chunk(
            String::from("12345 x y z w x "),
            1,
            1,
            6
        );
        assert_eq!(*outp.front().unwrap(), "x")
    }
}