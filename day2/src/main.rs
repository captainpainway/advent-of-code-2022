use std::fs;
use std::collections::HashMap;
use std::str::SplitTerminator;

#[derive(Hash, Eq, PartialEq, Clone)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn get_opponent_choice(letter: char) -> Result<Self, String> {
        match letter {
            'A' => Result::Ok(Self::Rock),
            'B' => Result::Ok(Self::Paper),
            'C' => Result::Ok(Self::Scissors),
            _ => Result::Err(String::from("Invalid opponent choice")),
        }
    }

    fn get_player_choice(letter: char) -> Result<Self, String> {
        match letter {
            'X' => Result::Ok(Self::Rock),
            'Y' => Result::Ok(Self::Paper),
            'Z' => Result::Ok(Self::Scissors),
            _ => Result::Err(String::from("Invalid player choice")),
        }
    }
}

#[derive(Hash, Eq, PartialEq)]
enum Strategy {
    Lose,
    Draw,
    Win,
}

impl Strategy {
    fn get_strategy(letter: char) -> Result<Self, String> {
        match letter {
            'X' => Result::Ok(Self::Lose),
            'Y' => Result::Ok(Self::Draw),
            'Z' => Result::Ok(Self::Win),
            _ => Result::Err(String::from("Invalid strategy")),
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap_or("".to_string());
    let arr = contents.split_terminator("\n");

    let values = HashMap::from([
        (Choice::Rock, 1),
        (Choice::Paper, 2),
        (Choice::Scissors, 3),
    ]);

    strategy_1(arr.clone(), values.clone());
    strategy_2(arr, values);
}

fn strategy_1(arr: SplitTerminator<&str>, values: HashMap<Choice, i32>) {
    let scores = HashMap::from([
        (Choice::Rock, HashMap::from([
            (Choice::Rock, 3), // Rock draws
            (Choice::Paper, 6), // Paper wins
            (Choice::Scissors, 0), // Scissors loses
            ]),
        ),
        (Choice::Paper, HashMap::from([
            (Choice::Rock, 0), // Rock loses
            (Choice::Paper, 3), // Paper draws
            (Choice::Scissors, 6), // Scissors wins
            ]),
        ),
        (Choice::Scissors, HashMap::from([
            (Choice::Rock, 6), // Rock wins
            (Choice::Paper, 0), // Paper loses
            (Choice::Scissors, 3), // Scissors draws
            ]),
        ),
    ]);

    let mut total = 0;
    for round in arr {
        let opponent = Choice::get_opponent_choice(round.chars().nth(0).unwrap()).unwrap();
        let player = Choice::get_player_choice(round.chars().nth(2).unwrap()).unwrap();
        let score = scores[&opponent][&player];
        total += score + values[&player];
    }
    println!("{:?}", total);
}

fn strategy_2(arr: SplitTerminator<&str>, values: HashMap<Choice, i32>) {
    let selections = HashMap::from([
        (Choice::Rock, HashMap::from([
            (Strategy::Lose, Choice::Scissors), // To lose, select Scissors
            (Strategy::Draw, Choice::Rock), // To draw, select Rock
            (Strategy::Win, Choice::Paper), // To win, select Paper
            ]),
        ),
        (Choice::Paper, HashMap::from([
            (Strategy::Lose, Choice::Rock), // To lose, select Rock
            (Strategy::Draw, Choice::Paper), // To draw, select Paper
            (Strategy::Win, Choice::Scissors), // To win, select Scissors
            ]),
        ),
        (Choice::Scissors, HashMap::from([
            (Strategy::Lose, Choice::Paper), // To lose, select Paper
            (Strategy::Draw, Choice::Scissors), // To draw, select Scissors
            (Strategy::Win, Choice::Rock), // To win, select Rock
            ]),
        ),
    ]);

    let scores = HashMap::from([
        (Strategy::Lose, 0),
        (Strategy::Draw, 3),
        (Strategy::Win, 6),
    ]);

    let mut total = 0;
    for round in arr {
        let opponent = Choice::get_opponent_choice(round.chars().nth(0).unwrap()).unwrap();
        let player = Strategy::get_strategy(round.chars().nth(2).unwrap()).unwrap();
        let selection = &selections[&opponent][&player];
        total += scores[&player] + values[&selection];
    }
    println!("{:?}", total);
}
