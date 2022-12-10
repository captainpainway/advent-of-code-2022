use std::fs;
use std::collections::HashMap;
use std::cmp;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let arr: Vec<&str> = input.split_terminator("\n").collect();
    let smallest_sum = find_smallest_dir_sum(
        directory_sizes(arr.clone())
    );
    println!("The sum of the smallest directories is {}", smallest_sum);
    let to_delete = find_smallest_dir_to_delete(
        directory_sizes(arr)
    );
    println!("The smallest directory to delete is {}", to_delete);
}

fn directory_sizes(lines: Vec<&str>) -> HashMap<String, i32> {
    let mut dir_map: HashMap<String, i32> = HashMap::new();
    let mut current_dirs: Vec<&str> = Vec::new();
    for line in lines {
        let split: Vec<&str> = line.split(" ").collect();
        if split[0] == "$" {
            if line != "$ ls" { // We don't care about this commmand.
                let dir = split[2];
                // Create a list of all dirs we're in right now.
                if dir != ".." {
                    current_dirs.push(dir);
                } else {
                    // When "cd ..", remove the most recent dir from the list.
                    current_dirs.pop();
                }
            }
        } else {
            // Add directory and file sizes to the hash map.
            let size: i32 = split[0]
                .parse()
                .unwrap_or(0);
            for (i, _) in current_dirs.iter().enumerate() {
                // Directories within other directories could have the
                // same name, so create a unique key with all the
                // directory names in the path.
                let key = current_dirs[..i + 1].join("");
                *dir_map.entry(key).or_insert(0) += size;
            }
        }
    }
    dir_map
}

fn find_smallest_dir_sum(dirs: HashMap<String, i32>) -> i32 {
    dirs.values().fold(0, |acc, val| {
        if val <= &100000 {
            return acc + val;
        }
        return acc + 0;
    })
}

fn find_smallest_dir_to_delete(dirs: HashMap<String, i32>) -> i32 {
    let used_space = dirs.get("/").unwrap();
    let unused_space = 70000000 - used_space;
    let needed_space = 30000000 - unused_space;
    let mut smallest = used_space;
    for dir in dirs.values() {
        if *dir >= needed_space {
            smallest = cmp::min(dir, smallest);
        }
    }
    *smallest
}
