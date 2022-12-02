use std::fs;
use std::collections::HashMap;
use std::str::SplitTerminator;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap_or("".to_string());
    let arr = contents.split_terminator("\n");

    let values = HashMap::from([
        ('X', 1), // Rock is worth 1
        ('Y', 2), // Paper is worth 2
        ('Z', 3), // Scissors is worth 3
    ]);

    strategy_1(arr.clone(), values.clone());
    strategy_2(arr, values);
}

fn strategy_1(arr: SplitTerminator<&str>, values: HashMap<char, i32>) {
    let scores = HashMap::from([
        ('A', HashMap::from([ // Rock
            ('X', 3), // Rock draws
            ('Y', 6), // Paper wins
            ('Z', 0), // Scissors loses
            ]),
        ),
        ('B', HashMap::from([ // Paper
            ('X', 0), // Rock loses
            ('Y', 3), // Paper draws
            ('Z', 6), // Scissors wins
            ]),
        ),
        ('C', HashMap::from([ // Scissors
            ('X', 6), // Rock wins
            ('Y', 0), // Paper loses
            ('Z', 3), // Scissors draws
            ]),
        ),
    ]);

    let mut total = 0;
    for round in arr {
        let opponent = round.chars().nth(0).unwrap();
        let player = round.chars().nth(2).unwrap();
        let score = scores[&opponent][&player];
        total += score + values[&player];
    }
    println!("{:?}", total);
}

fn strategy_2(arr: SplitTerminator<&str>, values: HashMap<char, i32>) {
    let selections = HashMap::from([
        ('A', HashMap::from([ // Rock
            ('X', 'Z'), // To lose, select Scissors
            ('Y', 'X'), // To draw, select Rock
            ('Z', 'Y'), // To win, select Paper
            ]),
        ),
        ('B', HashMap::from([ // Paper
            ('X', 'X'), // To lose, select Rock
            ('Y', 'Y'), // To draw, select Paper
            ('Z', 'Z'), // To win, select Scissors
            ]),
        ),
        ('C', HashMap::from([ // Scissors
            ('X', 'Y'), // To lose, select Paper
            ('Y', 'Z'), // To draw, select Scissors
            ('Z', 'X'), // To win, select Rock
            ]),
        ),
    ]);

    let scores = HashMap::from([
        ('X', 0), // Loss is worth 0
        ('Y', 3), // Draw is worth 3
        ('Z', 6), // Win is worth 6
    ]);

    let mut total = 0;
    for round in arr {
        let opponent = round.chars().nth(0).unwrap();
        let player = round.chars().nth(2).unwrap();
        let selection = selections[&opponent][&player];
        total += scores[&player] + values[&selection];
    }
    println!("{:?}", total);
}
