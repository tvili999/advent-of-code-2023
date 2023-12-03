use std::fs::read_to_string;

fn main() {
    let input_raw = read_to_string("input.txt").unwrap();

    let mut sum: u64 = 0;

    for line in input_raw.lines() {
        let digits: Vec<char> = line.chars().filter(|ch| ch.is_digit(10)).collect();

        let number = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());

        sum += number.parse::<u64>().unwrap()
    }

    println!("{}", sum);
}
