use std::{collections::HashMap, fs};

use types::{CubeColor, Game};

mod types;

fn main() {
    let mut sum: u64 = 0;
    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let mut maxes: HashMap<CubeColor, u16> = HashMap::new();

        let game = Game::parse(line.to_string());
        for reveal in game.reveals.iter() {
            for item in reveal.items.iter() {
                let color_max = maxes.get(&item.color).unwrap_or(&0);
                if item.quantity > *color_max {
                    maxes.insert(item.color, item.quantity);
                }
            }
        }

        let power: u64 = maxes.values().fold(1, |a, b| a * (*b as u64));
        sum += power;
    }

    println!("{}", sum);
}
