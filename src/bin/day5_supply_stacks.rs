use std::collections::VecDeque;
use std::ptr::addr_of_mut;
use advent_of_code2022::utils::aoc_traits_alg::*;
use advent_of_code2022::utils::string_processing::*;

struct MoveSupplyAction {
    number: u64,
    source: u64,
    dest: u64
}

impl AoCLineRepObject for MoveSupplyAction{
    fn build_from_line(string: String) -> Self {
        let numeric_values: VecDeque<i64> = pull_all_numbers(string);

        MoveSupplyAction {
            number: *numeric_values.get(0).expect("No 3rd value found") as u64,
            source: *numeric_values.get(1).expect("No 2nd value found") as u64,
            dest: *numeric_values.get(2).expect("No 1st value found") as u64
        }
    }
}

struct CrateStack {
    create_stacks: Vec<Vec<String>>,
    actions: VecDeque<MoveSupplyAction>
}

impl AoCMultiLineRepObject for CrateStack {
    fn build_from_string_vec_deque(string_vector: VecDeque<String>) -> Self {
        let mut crate_data: VecDeque<String> = VecDeque::new();
        let mut position = 0;
        loop {
            let cur_string = string_vector.get(position).unwrap().clone();
            position += 1;
            if pull_all_words(cur_string.clone()).is_empty() {
                crate_data.pop_front();
                break
            }
            crate_data.push_front(cur_string);
        }

        let mut crates: Vec<Vec<String>> = Vec::new();
        for item in crate_data {
            let row_contents = chunk(item,3, 1, 0);
            let mut row_position = 0;
            for item in row_contents {
                if item != "   " {
                    while crates.len() <= row_position {
                        crates.push(Vec::new());
                    }
                    if row_position < crates.len() { crates[row_position].push(
                        String::from(item.chars().nth(1).unwrap())
                    )}
                }
                row_position += 1
            }
        }

        let mut action_que: VecDeque<String> = VecDeque::new();
        while position < string_vector.len() {
            action_que.push_back(string_vector.get(position).unwrap().clone());
            position += 1;
        }

        CrateStack {
            create_stacks: crates,
            actions: build_from_string_vec_deque(action_que)
        }
    }
}

impl CrateStack {
    fn preform_action(&mut self, action: MoveSupplyAction, is_single: bool) {
        let mut item_que: VecDeque<String> = VecDeque::new();
        for _ in 0..action.number {
            let mut src_top = self.create_stacks[action.source as usize - 1].pop().unwrap();
            if is_single {item_que.push_back(src_top)} else {item_que.push_front(src_top)}
        }
        for item in item_que {self.create_stacks[action.dest as usize - 1].push(item)}
    }

    fn preform_actions(&mut self, is_single: bool) {
        while !self.actions.is_empty() {
            let next_action: MoveSupplyAction = self.actions.pop_front().unwrap();
            self.preform_action(next_action, is_single)
        }
    }

    fn get_peak(&self) -> String {
        let mut peak = String::new();
        for index in 0..self.create_stacks.len() {
            let mut _end = self.create_stacks[index].len() as i64 - 1;
            if _end >= 0 { peak.push_str(self.create_stacks[index][_end as usize].as_str()) }
            else { peak.push_str(" ") }
        }
        peak
    }
}

pub(crate) fn main() {
    let mut create_stack: CrateStack = build_from_file(&String::from("test_inputs/Day5Part1Example.txt"));
    create_stack.preform_actions(true);
    println!("{}", create_stack.get_peak());
    let mut create_stack: CrateStack = build_from_file(&String::from("inputs/Day5Part1.txt"));
    create_stack.preform_actions(true);
    println!("{}", create_stack.get_peak());
    let mut create_stack: CrateStack = build_from_file(&String::from("test_inputs/Day5Part1Example.txt"));
    create_stack.preform_actions(false);
    println!("{}", create_stack.get_peak());
    let mut create_stack: CrateStack = build_from_file(&String::from("inputs/Day5Part1.txt"));
    create_stack.preform_actions(false);
    println!("{}", create_stack.get_peak());
}

mod tests {
    use super::*;

    #[test]
    fn test_action() {
        let action = MoveSupplyAction{
            number: 1,
            source: 2,
            dest: 1
        };
        let mut create_stack: CrateStack = build_from_file(&String::from("test_inputs/Day5Part1Example.txt"));
        assert_eq!(create_stack.get_peak(), "NDP");
        create_stack.preform_action(action, true);
        assert_eq!(create_stack.get_peak(), "DCP");
    }

    #[test]
    fn test_actions() {
        let mut create_stack: CrateStack = build_from_file(&String::from("test_inputs/Day5Part1Example.txt"));
        create_stack.preform_actions(true);
        assert_eq!(create_stack.get_peak(), "CMZ")
    }
}