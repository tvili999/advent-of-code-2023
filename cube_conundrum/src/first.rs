use std::fs;

use types::{CubeColor, Game};

mod types;

fn limit(color: &CubeColor) -> u16 {
    match color {
        CubeColor::Red => 12,
        CubeColor::Green => 13,
        CubeColor::Blue => 14,
    }
}

fn possible(game: &Game) -> bool {
    for reveal in game.reveals.iter() {
        for item in reveal.items.iter() {
            if item.quantity > limit(&item.color) {
                return false;
            }
        }
    }
    return true;
}

fn main() {
    let mut sum = 0;
    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let game = Game::parse(line.to_string());
        if possible(&game) {
            sum += game.id;
        }
    }

    println!("{}", sum);
}
