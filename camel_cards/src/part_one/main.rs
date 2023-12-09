use std::fs;

use parser::Play;

mod hand;
mod parser;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut plays: Vec<Play> = input.lines().map(|line| Play::parse(line)).collect();

    plays.sort_by(|a, b| a.hand.cmp(&b.hand));

    let mut result = 0;
    for (rank, play) in plays.iter().enumerate() {
        println!("{} -> {:?}", play.hand.cards, play.hand.hand_type());

        result += (rank as u32 + 1) * play.bid;
    }

    println!("{}", result);
}
