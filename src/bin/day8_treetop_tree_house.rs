use std::collections::{VecDeque, HashSet};
use advent_of_code2022::utils::aoc_traits_alg::{AoCMultiLineRepObject, build_from_file};
use advent_of_code2022::utils::array_traits::*;

struct Forest{
    height: usize,
    width: usize,
    trees: VecDeque<VecDeque<u64>>
}

impl AoCMultiLineRepObject for Forest {
    fn build_from_string_vec_deque(string_vector: VecDeque<String>) -> Self {
        let mut trees: VecDeque<VecDeque<u64>> = VecDeque::new();
        for line in string_vector {
            let mut tree_line: VecDeque<u64> = VecDeque::new();
            let mut raw_trees = line.clone();
            while !raw_trees.is_empty() {
                tree_line.push_back(
                    String::from(raw_trees.remove(0)).parse::<u64>().unwrap()
                )
            }
            trees.push_back(tree_line)
        }
        Forest {
            height: trees.len(),
            width: trees[0].len(),
            trees
        }
    }
}

impl ArrayIndexOne for Forest {
    type Value = u64;

    fn get_dim(&self) -> (usize, usize) { return (self.height, self.width); }

    fn get_value(&self, position: &(usize, usize)) -> Self::Value {
        return self.trees[position.0 as usize][position.1 as usize].clone();
    }
}

impl Forest {
    fn get_number_visible(&self) -> usize {
        let mut values: HashSet<(i64, i64)> = HashSet::new();
        let directions: Vec<(i64,i64)> = vec![(1,0),(-1,0),(0,1),(0,-1)];
        for dir in directions {
            let highest = sweep_for_memo_direction(self,
                                                   dir.clone(), &(0,0),
                                                   Self::initializer,Self::part1_updater);
            let mut initial = get_final_positions(self,&dir);
            while !initial.is_empty() {
                let position = initial.pop_front().unwrap();
                values.insert(position.clone());
                let new_pos = get_value_in_vec_deque(self, &highest, &position);
                if !new_pos.is_none() && self.point_inbounds(&new_pos.clone().unwrap()) {
                    initial.push_front(new_pos.unwrap())
                }
            }
        }
        values.len()
    }

    fn get_max_scenic_score(&self) -> usize {
        let directions: Vec<(i64,i64)> = vec![(1,0), (-1,0),(0,1),(0,-1)];
        let mut sides: Vec<VecDeque<VecDeque<(i64, i64)>>> = Vec::new();
        for dir in directions {
            sides.push(
                sweep_for_memo_direction(self, dir.clone(), &(0,0),
                                         Self::initializer,Self::part2_updater)
            );
        }
        let mut max = 0;
        for i in 1..(self.height+1) {
            for j in 1..(self.width+1) {
                let up_side = (sides[0][i-1][j-1].0 - i as i64).abs() - !self.point_inbounds(&sides[0][i-1][j-1].clone()) as i64;
                let down_side = (sides[1][i-1][j-1].0 - i as i64).abs() - !self.point_inbounds(&sides[1][i-1][j-1].clone()) as i64;
                let left_side = (sides[2][i-1][j-1].1 - j as i64).abs() - !self.point_inbounds(&sides[2][i-1][j-1].clone()) as i64;
                let right_side = (sides[3][i-1][j-1].1 - j as i64).abs() - !self.point_inbounds(&sides[3][i-1][j-1].clone()) as i64;
                let new_value = up_side*down_side*left_side*right_side;
                if new_value > max {max = new_value}
            }
        }
        max as usize
    }

    fn initializer(memo: &mut VecDeque<VecDeque<(i64, i64)>>, input: &Forest,
                   pos: &(i64, i64), dir: &(i64, i64)){
        set_value_in_vec_deque(input, memo, pos,
                               (pos.0-dir.0,pos.1-dir.1));
    }

    fn part1_updater(memo: &mut VecDeque<VecDeque<(i64, i64)>>, input: &Forest,
                     prior_position: &(i64, i64), next_pos: &(i64,i64), dir: &(i64, i64)) {
        let mut prior_pos = prior_position.clone();
        let next_tree = input.take_value(next_pos);
        loop {
            let mut prior_tree = input.take_value(&prior_pos);
            if prior_tree.is_none() || prior_tree > next_tree {
                set_value_in_vec_deque(input, memo,
                                       next_pos, prior_pos.clone());
                break
            } else {
                prior_pos = get_value_in_vec_deque(input, memo, &prior_pos).unwrap();
                prior_tree = input.take_value(&prior_pos);
            }
        }
    }

    fn part2_updater(memo: &mut VecDeque<VecDeque<(i64, i64)>>, input: &Forest,
                     prior_position: &(i64, i64), next_pos: &(i64,i64), dir: &(i64, i64)) {
        let mut prior_pos = prior_position.clone();
        let next_tree = input.take_value(next_pos);
        loop {
            println!("{prior_pos:?}");
            let mut prior_tree = input.take_value(&prior_pos);
            if prior_tree.is_none() || prior_tree >= next_tree {
                set_value_in_vec_deque(input, memo,
                                       next_pos, prior_pos.clone());
                break
            } else {
                prior_pos = get_value_in_vec_deque(input, memo, &prior_pos).unwrap();
                prior_tree = input.take_value(&prior_pos);
            }
        }
    }

}

fn main() {
    let forest: Forest = build_from_file(&String::from("inputs/Day8Part1.txt"));
    println!("{}", forest.get_number_visible());
    println!("{}", forest.get_most_scenic());
}

mod tests {
    use super::*;

    #[test]
    fn test_find_visible() {
        let forest: Forest = build_from_file(&String::from("test_inputs/Day8Part1Example.txt"));
        assert_eq!(forest.get_number_visible(), 21);
    }

    #[test]
    fn test_get_scenic() {
        let forest: Forest = build_from_file(&String::from("test_inputs/Day8Part1Example.txt"));
        assert_eq!(forest.get_max_scenic_score(), 8);
    }
}