

mod day1_calorie_counting;

fn main() {
    let max_calories: i32 = day1_calorie_counting::find_most_calorie("inputs/Day1Part1.txt");
    println!("{max_calories}")
}
