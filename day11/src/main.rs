use std::fs;
use std::collections::{VecDeque, HashMap};

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
        let items = ins[1]
            .trim()
            .split(": ")
            .nth(1)
            .unwrap()
            .split(", ")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<VecDeque<i64>>();
        let operation = ins[2]
            .split(" = ")
            .nth(1)
            .unwrap()
            .to_string();
        let test: i64 = ins[3]
            .trim()
            .split(" ")
            .nth(3)
            .unwrap()
            .parse()
            .unwrap();
        Monkey {
            items,
            operation,
            test,
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

    fn inspect_items(&mut self) {
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
    }

    fn lose_interest(&mut self, interest: i64) {
        self.items = self.items.iter()
            .map(|item| item / interest)
            .collect::<VecDeque<i64>>();
    }

    fn manage_stress(&mut self, stress_relief: i64) {
        self.items = self.items.iter()
            .map(|item| item % stress_relief)
            .collect::<VecDeque<i64>>();
    }

    fn throw(&mut self) -> HashMap<usize, Vec<i64>> {
        let mut throws: HashMap<usize, Vec<i64>> = HashMap::new();
        while self.items.len() > 0 {
            let item = self.items.pop_front().unwrap();
            let target = match item % self.test == 0 {
                true => self.yes,
                _ => self.no
            };
            throws.entry(target).or_insert_with(Vec::new).push(item);
        }
        throws
    }
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
    let mut all_monkeys: Vec<Monkey> = Vec::new();
    for monkey in monkeys {
        let m = Monkey::new(monkey);
        all_monkeys.push(m);
    }
    let stress_relief: i64 = all_monkeys.iter().map(|m| m.test).product();
    for _ in 0..rounds {
        for i in 0..all_monkeys.len() {
            all_monkeys[i].inspect_items();
            all_monkeys[i].lose_interest(interest_lost);
            all_monkeys[i].manage_stress(stress_relief);
            let throws = all_monkeys[i].throw();
            for key in throws.keys() {
                let recipient = &mut all_monkeys[*key];
                for item in &throws[key] {
                    recipient.items.push_back(*item);
                }
            }
        }
    }
    let mut inspected = all_monkeys
        .into_iter()
        .map(|x| x.inspected)
        .collect::<Vec<i64>>();
    inspected.sort();
    inspected
        .iter()
        .rev()
        .take(2)
        .product::<i64>()
}