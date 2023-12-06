use std::fs;

#[derive(Debug)]
struct Round {
    time: i64,
    distance: i64,
}

fn parse_round(input: String) -> Round {
    let mut time: i64 = 0;
    let mut distance: i64 = 0;

    for line in input.lines() {
        let parts = line.split(':').map(|x| x.trim()).collect::<Vec<&str>>();
        let series_name = *parts.get(0).unwrap();
        let number: i64 = parts
            .get(1)
            .unwrap()
            .replace(" ", "")
            .parse::<i64>()
            .unwrap();

        if series_name == "Time" {
            time = number;
        }
        if series_name == "Distance" {
            distance = number;
        }
    }

    Round { time, distance }
}

fn solve_quadratic(a: f64, b: f64, c: f64) -> (f64, f64) {
    let d = (b * b - 4.0 * a * c).sqrt();

    let x1 = (-b - d) / (2.0 * a);
    let x2 = (-b + d) / (2.0 * a);

    (x1, x2)
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let round = parse_round(input);

    let mut prod = 1;
    let (x1, x2) = solve_quadratic(
        -1.0,
        round.time as f64,
        -1.0 * round.distance as f64 - 0.0000000000001,
    );
    let ways = x1.floor() - x2.ceil() + 1.0;
    println!("{} - {} -> {}", x1, x2, ways);

    prod *= ways as i32;

    println!("{}", prod);
}
