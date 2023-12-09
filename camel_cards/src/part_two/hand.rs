use std::cmp::Ordering;

const JOKER_INDEX: usize = 0;
static CARDS: &str = "J23456789TQKA";

fn get_card_strength(card: char) -> usize {
    for (i, ch) in CARDS.chars().enumerate() {
        if ch == card {
            return i;
        }
    }
    panic!("No such card {}", card);
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub struct Hand {
    pub cards: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    pub fn new(cards: &str) -> Self {
        Self {
            cards: cards.to_string(),
        }
    }

    pub fn to_vec(&self) -> Vec<usize> {
        let mut vec: Vec<usize> = Vec::new();

        let mut best_card_index = 1;
        let mut best_card_num = 0;
        for (i, ch) in CARDS.chars().enumerate() {
            let mut card_num: usize = 0;

            for card in self.cards.chars() {
                if card == ch {
                    card_num += 1;
                }
            }

            if card_num > best_card_num && i != JOKER_INDEX {
                best_card_index = i;
                best_card_num = card_num;
            }

            vec.push(card_num);
        }

        if best_card_index != JOKER_INDEX {
            vec[best_card_index] += vec[JOKER_INDEX];
            vec[JOKER_INDEX] = 0;
        }

        vec
    }

    pub fn hand_type(&self) -> HandType {
        let potential: Vec<usize> = self.to_vec().iter().filter(|&x| *x > 1).copied().collect();

        if potential.len() == 1 {
            let a = *potential.get(0).unwrap();
            if a == 5 {
                return HandType::FiveOfAKind;
            };
            if a == 4 {
                return HandType::FourOfAKind;
            };
            if a == 3 {
                return HandType::ThreeOfAKind;
            };
            if a == 2 {
                return HandType::OnePair;
            };
        }
        if potential.len() == 2 {
            let a = *potential.get(0).unwrap();
            let b = *potential.get(1).unwrap();
            if (a == 3 && b == 2) || (a == 2 && b == 3) {
                return HandType::FullHouse;
            }
            if a == 2 && b == 2 {
                return HandType::TwoPair;
            }
        }

        HandType::HighCard
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type() < other.hand_type() {
            return Ordering::Less;
        }
        if self.hand_type() > other.hand_type() {
            return Ordering::Greater;
        }

        for i in 0..5 {
            let my_card = *self.cards.chars().collect::<Vec<char>>().get(i).unwrap();
            let other_card = *other.cards.chars().collect::<Vec<char>>().get(i).unwrap();

            if get_card_strength(my_card) < get_card_strength(other_card) {
                return Ordering::Less;
            }
            if get_card_strength(my_card) > get_card_strength(other_card) {
                return Ordering::Greater;
            }
        }

        return Ordering::Equal;
    }
}
