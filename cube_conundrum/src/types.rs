#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum CubeColor {
    Blue,
    Red,
    Green,
}

impl CubeColor {
    pub fn parse(raw: String) -> Self {
        match raw.as_str() {
            "red" => CubeColor::Red,
            "green" => CubeColor::Green,
            "blue" => CubeColor::Blue,
            _ => panic!("Wrong color"),
        }
    }
}

#[derive(Debug)]
pub struct RevealItem {
    pub color: CubeColor,
    pub quantity: u16,
}

impl RevealItem {
    pub fn parse(raw: String) -> Self {
        let qty_color: Vec<&str> = raw.splitn(2, " ").collect();

        let quantity = qty_color.get(0).unwrap().parse::<u16>().unwrap();
        let color = CubeColor::parse(qty_color.get(1).unwrap().to_string());

        RevealItem { color, quantity }
    }
}

#[derive(Debug)]
pub struct Reveal {
    pub items: Vec<RevealItem>,
}

impl Reveal {
    pub fn parse(raw: String) -> Self {
        let mut items = Vec::new();

        for item_raw in raw.split(",") {
            items.push(RevealItem::parse(item_raw.trim().to_string()));
        }

        Reveal { items }
    }
}

#[derive(Debug)]
pub struct Game {
    pub id: u16,
    pub reveals: Vec<Reveal>,
}

impl Game {
    pub fn parse(raw: String) -> Self {
        let game_reveals: Vec<&str> = raw.splitn(2, ":").collect();

        let id: u16 = game_reveals
            .get(0)
            .unwrap()
            .replace("Game", "")
            .trim()
            .parse::<u16>()
            .unwrap();

        let mut reveals = Vec::new();

        for reveal_raw in game_reveals.get(1).unwrap().split(";") {
            reveals.push(Reveal::parse(reveal_raw.trim().to_string()));
        }

        Game { id, reveals }
    }
}
