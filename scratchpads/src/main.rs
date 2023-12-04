use std::fs;

use crate::game::Game;

mod game;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut games: Vec<Game> = Vec::new();
    let mut copies: Vec<u32> = Vec::new();
    for line in input.lines() {
        let game = Game::parse(line.to_string());
        games.push(game);
        copies.push(1);
    }

    for (i, game) in games.iter().enumerate() {
        let overlapping: Vec<u32> = game
            .winner_numbers
            .iter()
            .filter(|num| game.my_numbers.contains(num))
            .map(|&x| x)
            .collect();

        for j in (i + 1)..(i + 1 + overlapping.len()) {
            copies[j] += copies[i];
        }
    }

    let sum = copies.iter().copied().reduce(|a, b| a + b).unwrap();

    println!("{:#?}", copies);
    println!("{}", sum);
}
