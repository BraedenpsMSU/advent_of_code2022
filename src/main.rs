

mod day1_calorie_counting;

fn main() {
    let max_calories: i32 = day1_calorie_counting::find_most_calorie_held_by_elf("inputs/Day1Part1.txt");
    println!("{max_calories}");
    let max_calories: i32 = day1_calorie_counting::find_sum_of_3_best_elves("inputs/Day1Part1.txt");
    println!("{max_calories}")
}
