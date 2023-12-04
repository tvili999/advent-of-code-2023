pub struct Game {
    pub id: u32,
    pub winner_numbers: Vec<u32>,
    pub my_numbers: Vec<u32>,
}

impl Game {
    pub fn parse(input: String) -> Self {
        let parts: Vec<&str> = input.split(":").collect();

        let game_id: u32 = parts
            .get(0)
            .unwrap()
            .split(" ")
            .filter(|&x| !x.is_empty())
            .collect::<Vec<&str>>()
            .get(1)
            .unwrap()
            .parse::<u32>()
            .unwrap();

        let numbers: Vec<&str> = parts.get(1).unwrap().split("|").collect();

        let winner_numbers: Vec<u32> = numbers
            .get(0)
            .unwrap()
            .split(" ")
            .filter(|&x| !x.is_empty())
            .map(|x| x.trim().parse::<u32>().unwrap())
            .collect();

        let my_numbers: Vec<u32> = numbers
            .get(1)
            .unwrap()
            .split(" ")
            .filter(|&x| !x.is_empty())
            .map(|x| x.trim().parse::<u32>().unwrap())
            .collect();

        Self {
            id: game_id,
            winner_numbers,
            my_numbers,
        }
    }
}
