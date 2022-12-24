use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

#[derive(Debug)]
struct FileNode {
    name: String,
    parent: Option<String>,
    child: Vec<String>,
    size: u64,
    is_file: bool
}

impl FileNode {
    fn get_path(&self) -> String{
        let mut path = String::from("");
        let parent_path = self.parent.clone().unwrap_or(
            String::from("")
        );
        path.push_str(parent_path.as_str());
        path.push_str(self.name.as_str());
        path.push_str("/");
        path
    }

    fn has_child(&self, suffix: String) -> bool{
        let path = format!("{}{}/", self.get_path(), suffix);
        self.child.contains(&path)
    }

    fn get_nice_str(&self) -> String {
        let mut is_file_marker = "X";
        if !self.is_file {
            is_file_marker = " ";
        }
        return format!("{}\t{}\t{}", is_file_marker, self.get_path(), self.size)
    }
}

struct FileExplorer {
    root: String,
    current_node: String,
    file_system: HashMap<String, FileNode>
}

impl FileExplorer {
    fn initialize(root_name: String) -> FileExplorer {
        let mut map: HashMap<String, FileNode> = HashMap::new();
        map.insert(format!("{}/", root_name.clone()),
                   FileNode {
                        name: root_name.clone(),
                        parent: None,
                        child: Vec::new(),
                        size: 0,
                        is_file: false
                   }
        );
        return FileExplorer{
            root: format!("{}/", root_name.clone()),
            current_node: format!("{}/", root_name.clone()),
            file_system: map
        }
    }

    fn add_directory(&mut self, suffix: String, is_file: bool){
        let mut current_node = self.file_system
            .get_mut(&self.current_node).unwrap();
        let current_path = current_node.get_path();
        current_node.child.push(
            format!("{}{}/", current_path.clone(), suffix.clone()
            )
        );
        self.file_system.insert(
            format!("{}{}/", current_path.clone(), suffix.clone()),
            FileNode {
                    name: suffix.clone(),
                    parent: Some(current_path),
                    child: Vec::new(),
                    size: 0,
                    is_file
            }
        );
    }

    fn move_back(&mut self) {
        let mut get_node:&FileNode = self.file_system
            .get_mut(&self.current_node)
            .unwrap();
        let parent_name = get_node.parent.clone();
        self.current_node = parent_name.expect("This can not be called on root");
    }

    fn move_to(&mut self, name: String, is_file: bool) {
        let mut get_node:&FileNode = self.file_system
            .get_mut(&self.current_node)
            .unwrap();
        if !get_node.has_child(name.clone()) {
            self.add_directory(name.clone(),is_file);
        }
        let current_node = self.file_system
            .get(&self.current_node).unwrap();
        self.current_node = format!("{}{}/", current_node.get_path(), name.clone()
        )
    }

    fn increase_size(&mut self, size: u64) {
        let mut get_current:&mut FileNode = self.file_system
            .get_mut(&self.current_node)
            .unwrap();
        loop {
            get_current.size += size;
            let parent_name = get_current.parent.clone();
            match parent_name {
                None => break,
                Some(i) => {
                    get_current = self.file_system
                    .get_mut(&i)
                    .expect("Failed to get current node")}
            }
        }
    }

    fn print_nicely(&self) {
        println!(" \t{}\t{}", self.root, self.file_system.get(self.root.as_str()).unwrap().size);
        self.print_nicely_driver(self.root.clone());
    }

    fn print_nicely_driver(&self, location: String){
        for item in self.file_system.get(location.as_str()).unwrap().child.clone() {
            println!("{}", self.file_system.get(item.as_str()).unwrap().get_nice_str());
            self.print_nicely_driver(item.clone());
        }
    }

    fn get_below_number_of_directories_threshold(&self, threshold: u64) -> u64{
        self.get_below_driver(self.root.clone(), threshold)
    }

    fn get_below_driver(&self, location: String, threshold:u64) -> u64 {
        let mut output: u64 = 0;
        if (self.file_system.get(location.as_str()).unwrap().size <= threshold) &&
            !self.file_system.get(location.as_str()).unwrap().is_file {
            output += self.file_system.get(location.as_str()).unwrap().size;
        }
        for item in self.file_system.get(location.as_str()).unwrap().child.clone() {
            output += self.get_below_driver(item.clone(), threshold);
        }
        output
    }

    fn get_smallest(&self, lower_bound: u64) -> String{
        self.get_smallest_driver(self.root.clone(), lower_bound)
    }

    fn get_smallest_driver(&self, location: String, lower_bound: u64) -> String {
        let mut output = self.root.clone();
        let current_best = self.file_system.get(output.as_str()).unwrap().size;
        // run on current location
        let mut temp_value = self.file_system.get(location.as_str()).unwrap().size;
        if temp_value < current_best && temp_value > lower_bound {
            output = location.clone();
        }
        // run on children
        let mut current_output: String = String::from("");
        for item in self.file_system.get(location.clone().as_str()).unwrap().child.clone() {
            current_output = self.get_smallest_driver(item.clone(), lower_bound);
            temp_value = self.file_system.get(current_output.as_str()).unwrap().size;
            if temp_value < current_best {
                output = current_output;
            }
        }
        output
    }

}

fn get_fs(file_path: &str) -> FileExplorer {
    let file = File::open(file_path).expect("Cannot open.txt");
    let reader = BufReader::new(file);
    let mut cmd_store: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line_info = line.expect("Something went wrong when reading in a line");
        cmd_store.insert(0,line_info.clone());
    }

    let mut current_system: Option<FileExplorer> = None;
    while !cmd_store.is_empty() {
        let action_string: String = cmd_store.pop()
            .unwrap();
        let action: Vec<String> = action_string.split(' ')
            .into_iter()
            .map(|x| String::from(x))
            .collect();
        if current_system.is_none() {
            current_system = Some(FileExplorer::initialize(String::from("")));
        }
        else {
            let mut system = current_system.unwrap();
            if action[0] == "$" {
                if action[1] == "ls" {}
                else if action[1] == "cd" {
                    if action[2] == ".." {
                        system.move_back();
                    } else {
                        system.move_to(String::from(action[2].clone()), false);
                    }
                }
            } else if action[0] == "dir" {
                system.add_directory(String::from(action[1].clone()), false)
            } else if  String::from(action[0].clone()).parse::<u64>().is_ok() {
                system.move_to(String::from(action[1].clone()), true);
                system.increase_size(String::from(action[0].clone())
                    .parse::<u64>().expect("invalid integer")
                );
                system.move_back();
            }
            current_system = Some(system);
        }
    }
    let final_system = current_system.unwrap();
    final_system
}

pub fn get_fs_size(file_path: &str) -> u64 {
    let fs = get_fs(file_path);
    fs.get_below_number_of_directories_threshold(100000)
}

pub fn get_smallest_size(file_path: &str) -> u64 {
    let fs = get_fs(file_path);
    let root_size = fs.file_system.get(fs.root.as_str()).unwrap().size;
    let smallest_node = fs.get_smallest(
        30000000 + root_size - 70000000
    );
    fs.file_system.get(smallest_node.as_str()).unwrap().size
}

pub fn main() {
    let number_above_threshold = get_fs_size("inputs/Day7Part1.txt");
    println!("{number_above_threshold}");
    let number_above_threshold = get_smallest_size("inputs/Day7Part1.txt");
    println!("{number_above_threshold}");
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_example_part1() {
        let result = get_fs_size("test_inputs/Day7Part1Example.txt");
        assert_eq!(result, 95437);
    }
}