use std::fs;

#[derive(Clone)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let arr: Vec<&str> = input.split_terminator("\n\n").collect();
    let stacks = create_stacks(arr[0]);
    let instructions = create_instructions(arr[1]);
    let top = get_top_crates(
        move_crates(stacks.clone(), instructions.clone())
    );
    println!("The top crates on the stacks are: \"{}\"", top);
    let stacked = get_top_crates(
        move_stacks(stacks, instructions)
    );
    println!("Top crates after moving stacks are: \"{}\"", stacked);
}

/*
 * Parses the first part of the input to create stacks of crates
 */
fn create_stacks(line: &str) -> Vec<Vec<char>> {
    let mut lines: Vec<&str> = line.split("\n").collect();
    let max = lines
        .pop()
        .unwrap()
        .chars()
        .rev()
        .nth(1)
        .unwrap()
        .to_digit(10)
        .unwrap();
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut idx = 1;
    for _i in 0..max {
        let mut col: Vec<char> = Vec::new();
        for line in &lines {
            let ch = line.chars().nth(idx).unwrap();
            if ch != ' ' {
                col.push(ch);
            }
        }
        let reversed: Vec<char> = col.into_iter().rev().collect();
        stacks.push(reversed);
        idx += 4;
    }
    stacks
}

/*
 * Parses the second part of the input to create the instructions
 */
fn create_instructions(line: &str) -> Vec<Instruction> {
    line.split_terminator("\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|l| l.split(" ").collect::<Vec<&str>>())
        .into_iter()
        .map(|l| Instruction {
            amount: l[1].parse::<usize>().unwrap(),
            from: l[3].parse::<usize>().unwrap() - 1,
            to: l[5].parse::<usize>().unwrap() - 1,
        }).collect::<Vec<Instruction>>()
}

/*
 * Get the top crate off each stack and return as one string
 */
fn get_top_crates(stacks: Vec<Vec<char>>) -> String {
    stacks
        .iter()
        .map(|s| match s.len() {
            0 => " ".to_string(),
            n => s[n - 1].to_string(),
        }).collect::<Vec<String>>().join("")
}

/*
 * Part 1
 * Move crates one by one per instructions
 */
fn move_crates(mut stacks: Vec<Vec<char>>, instructions: Vec<Instruction>) -> Vec<Vec<char>> {
    for ins in instructions {
        for _ in 0..ins.amount {
            let c = stacks[ins.from].pop().unwrap();
            stacks[ins.to].push(c);
        }
    }
    stacks
}

/*
 * Part 2
 * Move crates in chunks per instructions
 */
fn move_stacks(mut stacks: Vec<Vec<char>>, instructions: Vec<Instruction>) -> Vec<Vec<char>> {
    for ins in instructions {
        let st_l = stacks[ins.from].len();
        let mut c = stacks[ins.from].split_off(st_l - ins.amount);
        stacks[ins.to].append(&mut c);
    }
    stacks
}
