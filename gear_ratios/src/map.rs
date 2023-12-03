use crate::tokens::Token;

fn get_size(tokens: &Vec<Token>) -> (usize, usize) {
    let first_token = tokens.first().unwrap();
    let mut w = first_token.x + first_token.value.len();
    let mut h = first_token.y;

    for token in tokens.iter() {
        if (token.x + token.value.len()) > w {
            w = token.x + token.value.len();
        }
        if token.y > h {
            h = token.y;
        }
    }

    return (w, h);
}

pub struct Map {
    pub w: usize,
    pub h: usize,
    pub map: Vec<Vec<Option<usize>>>,
}

impl Map {
    pub fn print_map(&self) {
        for line in self.map.iter() {
            for node in line.iter() {
                if let Some(i) = node {
                    print!("[{:2}]", i);
                } else {
                    print!("[  ]");
                }
            }
            println!();
        }
    }

    pub fn from_tokens(tokens: &Vec<Token>) -> Self {
        let mut map = Vec::new();

        let (w, h) = get_size(&tokens);

        for _ in 0..(h + 1) {
            let mut row = Vec::new();

            for _ in 0..(w + 1) {
                row.push(None);
            }

            map.push(row);
        }

        for (i, token) in tokens.iter().enumerate() {
            let l = token.x;
            let r = token.x + token.value.len();

            for x in l..r {
                map.get_mut(token.y).unwrap()[x] = Some(i);
            }
        }

        Map { map, w, h }
    }

    pub fn get_around(&self, x: usize, y: usize) -> Vec<usize> {
        let mut res = Vec::new();

        for offset_x in -1i64..2i64 {
            for offset_y in -1i64..2i64 {
                if offset_x == 0 && offset_y == 0 {
                    continue;
                }

                let x = (x as i64) + offset_x;
                let y = (y as i64) + offset_y;
                if x < 0 || x > (self.w as i64) || y < 0 || y > (self.h as i64) {
                    continue;
                }

                let x = x as usize;
                let y = y as usize;

                if let Some(idx) = self.map.get(y).unwrap().get(x).unwrap() {
                    if !res.contains(idx) {
                        res.push(*idx);
                    }
                }
            }
        }

        return res;
    }
}
