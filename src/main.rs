use std::collections::HashMap;
use std::env;
use std::process;

fn validate_dice(dice: [i32; 5]) -> bool
{
    for i in 0..5 {
        if dice[i] > 6 || dice[i] < 1 {
            return false;
        }
    }

    return true;
}

fn top_score(dice: [i32; 5]) -> i32
{
    let mut dice_map = HashMap::new();
    let mut top_score: i32 = 0;
    for _i in 1..6 {
        dice_map.insert(_i, 0);
    }

    for _i in 1..6 {
        dice_map.insert(dice[_i - 1], dice_map[&dice[_i - 1]] + 1);
    }

    for _i in 1..6 {
        let score = dice_map[&_i] * _i;
        if top_score < score
        {
            top_score = score;
        }
    }

    return top_score
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

    if !validate_dice(dice) {
        println!("Valid dice faces are 1 - 6");
        process::exit(0x2);
    }

    println!("Top upper section scoring: {:?}", top_score(dice));
}
