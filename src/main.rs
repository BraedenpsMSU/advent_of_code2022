use crate::orcale::read_input_file_into_vector_string;

mod day1_calorie_counting;
mod day2_rock_paper_scissors;
mod orcale;

fn main() {
    let max_calories: i32 = day1_calorie_counting::find_most_calorie_held_by_elf("inputs/Day1Part1.txt");
    println!("{max_calories}");
    let max_calories: i32 = day1_calorie_counting::find_sum_of_3_best_elves("inputs/Day1Part1.txt");
    println!("{max_calories}");
    let score_total: i32 = day2_rock_paper_scissors::get_score("test_inputs/Day2Part1Example.txt");
    println!("{score_total}");
    // let score_total: i32 = day2_rock_paper_scissors::get_score("inputs/Day2Part1.txt");
    // println!("{score_total}")
}
