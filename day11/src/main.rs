use std::fs;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<i64>,
    operation: String,
    test: i64,
    yes: usize,
    no: usize,
    inspected: i64
}

impl Monkey {
    fn new(instructions: &str) -> Monkey {
        let ins: Vec<&str> = instructions.split("\n").collect();
        Monkey {
            items: ins[1]
                .trim()
                .split(": ")
                .nth(1)
                .unwrap()
                .split(", ")
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<VecDeque<i64>>(),
            operation: ins[2]
                .split(" = ")
                .nth(1)
                .unwrap()
                .to_string(),
            test: ins[3]
                .trim()
                .split(" ")
                .nth(3)
                .unwrap()
                .parse()
                .unwrap(),
            yes: ins[4]
                .chars()
                .collect::<Vec<char>>()
                .pop()
                .unwrap()
                .to_string()
                .parse::<usize>()
                .unwrap(),
            no: ins[5]
                .chars()
                .collect::<Vec<char>>()
                .pop()
                .unwrap()
                .to_string()
                .parse::<usize>()
                .unwrap(),
            inspected: 0
        }
    }

    fn inspect_items(&mut self) -> &mut Self {
        self.items = self.items.iter().map(|item| {
            self.inspected += 1;
            let ops: Vec<&str> = self.operation.split(" ").collect();
            match ops[1] {
                "*" => {
                    match ops[2] {
                        "old" => item * item,
                        _ => item * ops[2].parse::<i64>().unwrap()
                    }
                },
                "+" => {
                    match ops[2] {
                        "old" => item + item,
                        _ => item + ops[2].parse::<i64>().unwrap()
                    }
                },
                _ => 0
            }
        }).collect::<VecDeque<i64>>();
        self
    }

    fn lose_interest(&mut self, interest: i64) -> &mut Self {
        self.items = self.items.iter()
            .map(|item| item / interest)
            .collect::<VecDeque<i64>>();
        self
    }

    fn manage_stress(&mut self, stress_relief: i64) -> &mut Self {
        self.items = self.items.iter()
            .map(|item| item % stress_relief)
            .collect::<VecDeque<i64>>();
        self
    }
}

fn throw(mut monkeys: Vec<Monkey>, i: usize) -> Vec<Monkey> {
    while monkeys[i].items.len() > 0 {
        let item = monkeys[i].items.pop_front().unwrap();
        let target = match item % monkeys[i].test == 0 {
            true => monkeys[i].yes,
            _ => monkeys[i].no
        };
        monkeys[target].items.push_back(item);
    }
    monkeys
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let monkeys: Vec<&str> = input.split_terminator("\n\n").collect();
    let business = monkey_business(monkeys.clone(), 20, 3);
    println!("There is {} monkey business after 20 rounds.", business);
    let more_business = monkey_business(monkeys, 10000, 1);
    println!("There is {} monkey business after 10000 rounds.", more_business);
}

fn monkey_business(monkeys: Vec<&str>, rounds: i64, interest_lost: i64) -> i64 {
    let mut all_monkeys: Vec<Monkey> = monkeys
        .iter()
        .map(|m| Monkey::new(m))
        .collect();
    let stress_relief: i64 = all_monkeys.iter().map(|m| m.test).product();
    for _ in 0..rounds {
        for i in 0..all_monkeys.len() {
            all_monkeys[i]
                .inspect_items()
                .lose_interest(interest_lost)
                .manage_stress(stress_relief);
            all_monkeys = throw(all_monkeys.clone(), i);
        }
    }
    let mut inspected = all_monkeys
        .into_iter()
        .map(|x| x.inspected)
        .collect::<Vec<i64>>();
    inspected.sort();
    inspected.iter().rev().take(2).product::<i64>()
}
