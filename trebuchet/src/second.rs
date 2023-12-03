use std::fs::read_to_string;

fn get_first_token(tokens: &Vec<&str>, line: &str) -> String {
    let mut i = line.len();
    let mut correct_token = String::new();

    for token in tokens.iter() {
        if let Some(index) = line.find(token) {
            println!("{}: {}", index, token);
            if index < i {
                i = index;
                correct_token = token.to_string()
            }
        }
    }

    return correct_token;
}

fn get_last_token(tokens: &Vec<&str>, line: &str) -> String {
    let mut i = 0;
    let mut correct_token = String::new();

    for token in tokens.iter() {
        if let Some(index) = line.rfind(token) {
            println!("{}: {}", index, token);
            if index > i {
                i = index;
                correct_token = token.to_string();
            }
        }
    }

    return correct_token;
}

fn main() {
    let input_raw = read_to_string("input.txt").unwrap();

    let mut sum: u64 = 0;

    let tokens = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];

    for line in input_raw.lines() {
        let first = get_first_token(&tokens, &line);
        let last = get_last_token(&tokens, &line);

        let token = format!("{}{}", first, last);
        println!("{}", line);
        println!("{}", token);
        println!("##########");
        let token = token.replace("one", "1");
        let token = token.replace("two", "2");
        let token = token.replace("three", "3");
        let token = token.replace("four", "4");
        let token = token.replace("five", "5");
        let token = token.replace("six", "6");
        let token = token.replace("seven", "7");
        let token = token.replace("eight", "8");
        let token = token.replace("nine", "9");

        let digits: Vec<char> = token.chars().filter(|ch| ch.is_digit(10)).collect();

        let number = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());

        sum += number.parse::<u64>().unwrap()
    }

    println!("{}", sum);
}
