use std::fs;

use crate::{map::Map, tokens::TokenType};

mod map;
mod tokens;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let tokens = tokens::parse_tokens(file.to_string());
    println!("{:#?}", tokens);

    let map = Map::from_tokens(&tokens);
    map.print_map();

    let mut visited_list: Vec<usize> = Vec::new();

    let mut sum = 0;

    for (y, row) in map.map.iter().enumerate() {
        for (x, pos) in row.iter().enumerate() {
            let token_idx = if let Some(i) = pos {
                i
            } else {
                continue;
            };

            let token = tokens.get(*token_idx).unwrap();

            if token.token_type != TokenType::Symbol {
                continue;
            }

            if token.value != String::from("*") {
                continue;
            }

            let around: Vec<usize> = map
                .get_around(x, y)
                .iter()
                .map(|&x| x)
                .filter(|&x| tokens.get(x).unwrap().token_type == TokenType::Value)
                .collect();

            if around.len() != 2 {
                continue;
            }

            let mut ratio = 1;

            for around_token_idx in around.iter() {
                // println!("token: {}", around_token_idx);
                let around_token = tokens.get(*around_token_idx).unwrap();

                if around_token.token_type != TokenType::Value {
                    continue;
                }

                let token_value = around_token.value.parse::<usize>().unwrap();
                ratio *= token_value;
            }

            sum += ratio;
        }
    }
    println!("{}", sum);
}
