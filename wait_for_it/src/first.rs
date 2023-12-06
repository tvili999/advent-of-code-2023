use std::fs;

#[derive(Debug)]
struct Round {
    time: i64,
    distance: i64,
}

fn parse_rounds(input: String) -> Vec<Round> {
    let mut times: Vec<i64> = Vec::new();
    let mut distances: Vec<i64> = Vec::new();

    let mut result = Vec::new();

    for line in input.lines() {
        let parts = line.split(':').map(|x| x.trim()).collect::<Vec<&str>>();
        let series_name = *parts.get(0).unwrap();
        let series: Vec<i64> = parts
            .get(1)
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        if series_name == "Time" {
            times.extend(series.iter())
        }
        if series_name == "Distance" {
            distances.extend(series.iter())
        }
    }

    for i in 0..times.len() {
        result.push(Round {
            time: *times.get(i).unwrap(),
            distance: *distances.get(i).unwrap(),
        })
    }

    result
}

fn solve_quadratic(a: f64, b: f64, c: f64) -> (f64, f64) {
    let d = (b * b - 4.0 * a * c).sqrt();

    let x1 = (-b - d) / (2.0 * a);
    let x2 = (-b + d) / (2.0 * a);

    (x1, x2)
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let rounds = parse_rounds(input);

    let mut prod = 1;
    for round in rounds {
        println!("Round");
        let (x1, x2) = solve_quadratic(
            -1.0,
            round.time as f64,
            -1.0 * round.distance as f64 - 0.0000000000001,
        );
        let ways = x1.floor() - x2.ceil() + 1.0;
        println!("{} - {} -> {}", x1, x2, ways);

        prod *= ways as i32;
    }

    println!("{}", prod);
}
