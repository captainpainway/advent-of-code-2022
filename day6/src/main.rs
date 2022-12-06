use std::fs;
use std::collections::HashSet;

fn main() {
    let input: Vec<char> = fs::read_to_string("input.txt").unwrap().trim().chars().collect();
    let start_marker = find_starts(input.clone(), 4);
    println!("The end of the packet sequence is at {}.", start_marker);
    let message_marker = find_starts(input, 14);
    println!("The end of the message sequence is at {}.", message_marker);
}

fn find_starts(input: Vec<char>, size: usize) -> usize {
    for (i, window) in input.windows(size).enumerate() {
        let set: HashSet<&char> = HashSet::from_iter(window.iter());
        if set.len() == size {
            return i + size;
        }
    }
    0
}
