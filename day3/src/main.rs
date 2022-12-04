use std::fs;
use std::str::SplitTerminator;
use std::collections::{HashMap, HashSet};

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let arr = contents.split_terminator("\n");
    let priority_sum = sum_priorities(
        find_dupes(
            rucksack_compartments(arr.clone())
        )
    );
    println!("{:?}", priority_sum);
    let group_sum = sum_priorities(
        find_badge(
            groups(arr.collect::<Vec<&str>>())
        )
    );
    println!("{:?}", group_sum);
}

/*
Divide rucksack into two compartments.
 */
fn rucksack_compartments<'a>(sacks: SplitTerminator<'a, &'a str>) -> Vec<(&'a str, &'a str)> {
    sacks.map(|sack| {
        let half = sack.len() / 2;
        return (&sack[..half], &sack[half..]);
    }).collect::<Vec<(&str, &str)>>()
}

/*
Find the duplicate item in the two compartments.
Creates a hash set from the first compartment
and checks the items in the second compartment against the set.
 */
fn find_dupes(sacks: Vec<(&str, &str)>) -> Vec<char> {
    sacks.iter().map(|sack| {
        let mut set = HashSet::new();
        let first_compartment = sack.0.chars();
        for item in first_compartment {
            set.insert(item);
        }
        let second_compartment = sack.1.chars();
        for item in second_compartment {
            if set.contains(&item) {
                return item;
            }
        }
        return ' ';
    }).collect::<Vec<char>>()
}

/*
Find the sum of the item priorities.
 */
fn sum_priorities(dupes: Vec<char>) -> usize {
    let values: [char; 52] = ['a', 'b', 'c', 'd', 'e', 'f', 'g',
    'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E',
    'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
    'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];

    dupes.iter().map(|item| {
        return values
            .iter()
            .position(|val| val == item)
            .unwrap() + 1;
    }).sum()
}

/*
Partition the list into groups of three.
 */
fn groups(sacks: Vec<&str>) -> Vec<Vec<&str>> {
    let mut groups: Vec<Vec<&str>> = Vec::new();
    for chunk in sacks.chunks(3) {
        groups.push(chunk.to_vec());
    }
    groups
}

/*
Find the badge common to the group.
Creates a hashmap, removes duplicates,
and checks for the only character that appears thrice.
 */
fn find_badge(groups: Vec<Vec<&str>>) -> Vec<char> {
    let mut badges: Vec<char> = Vec::new();
    for group in groups {
        let mut badge_map: HashMap<char, i32> = HashMap::new();
        for items in group {
            let mut dedup = items.chars().collect::<Vec<char>>();
            dedup.sort();
            dedup.dedup();
            for item in dedup {
                *badge_map.entry(item).or_insert(0) += 1;
            }
        }
        let common = badge_map.iter().find_map(|(key, &val)| {
            if val == 3 {
                Some(key)
            } else {
                None
            }
        });
        badges.push(*common.unwrap());
    }
    badges
}