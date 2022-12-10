use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    // Splitting the addx lines into two and flattening the array
    // allows us to use that first addx instruction as a noop,
    // since the addx instructions should take two cycles to complete.
    let arr: Vec<&str> = input
        .split_terminator("\n")
        .map(|x| x.split(" ").collect::<Vec<&str>>())
        .flatten()
        .collect();
    let signal_strength = signal_strengths(arr.clone());
    println!("The signal strength is {:?}", signal_strength);
    draw_pixels(arr);
}

fn signal_strengths(signals: Vec<&str>) -> i32 {
    let mut x = 1;
    let mut cycle = 1;
    let mut test = 20;
    let mut signal_strength = 0;
    for signal in signals {
        // Calculate the signal strength on certain cycles.
        // Do this before adding the new signal to x.
        if test == cycle {
            signal_strength += x * cycle;
            test += 40;
        }
        // If the input is a number, add it to x this cycle.
        let num = signal.parse::<i32>();
        match num {
            Ok(n) => x += n,
            Err(_) => ()
        }
        cycle += 1;
    }
    signal_strength
}

fn draw_pixels(signals: Vec<&str>) {
    let mut sprite_position = [0, 1, 2];
    let mut cycle = 0;
    let mut screen = [" "; 240];
    let mut line =  0;
    for signal in signals {
        // Add additional buffer to the sprite_position
        // to account for the length of the screen array.
        if cycle != 0 && cycle % 40 == 0 {
            line += 40;
        }

        // Write sprites to screen before repositioning sprite
        if sprite_position.map(|x| x + line).contains(&cycle) {
            screen[cycle as usize] = "#";
        }

        let num = signal.parse::<i32>();
        match num {
            Ok(n) => {
                // Update the sprite position array
                sprite_position = sprite_position
                    .map(|x| x + n)
            },
            Err(_) => ()
        };
        cycle += 1;
    }

    // Split the screen array into lines of 40 and print.
    let lines = screen.chunks(40);
    for line in lines {
        println!("{:?}", line.join(""));
    }
}