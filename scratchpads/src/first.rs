use std::fs;

use crate::game::Game;

mod game;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut sum = 0;
    for line in input.lines() {
        let game = Game::parse(line.to_string());

        let overlapping: Vec<u32> = game
            .winner_numbers
            .iter()
            .filter(|num| game.my_numbers.contains(num))
            .map(|&x| x)
            .collect();

        if overlapping.len() > 0 {
            sum += 2u32.pow(overlapping.len() as u32 - 1);
        }
    }

    println!("{}", sum);
}
