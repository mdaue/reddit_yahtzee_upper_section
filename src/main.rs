use std::env;
use std::process;

fn top_score(dice: [i32; 5]) -> i32
{
    let mut top_score: i32 = 0;
    for i in 1..6 {
        let score = i as i32 * dice[i - 1];
        if top_score < score {
            top_score = score;
        }
    }

    return top_score;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut dice: [i32; 5] = [0, 0, 0, 0, 0];

    if args.len() != 6
    {
        println!("Require 5 integers signifying the Yahtzee dice");
        process::exit(0x1);
    }

    for x in 0..5 {
        dice[x] = args[x + 1].parse::<i32>().unwrap();
    }

    println!("Top upper section scoring: {:?}", top_score(dice));
}
