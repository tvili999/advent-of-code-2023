use crate::hand::Hand;

pub struct Play {
    pub hand: Hand,
    pub bid: u32,
}

impl Play {
    pub fn parse(input: &str) -> Self {
        let parts: Vec<&str> = input.split_whitespace().collect();

        Self {
            hand: Hand::new(parts.get(0).unwrap()),
            bid: parts.get(1).unwrap().parse::<u32>().unwrap(),
        }
    }
}
