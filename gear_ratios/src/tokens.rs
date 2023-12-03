#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenType {
    Symbol,
    Value,
    None,
}

impl TokenType {
    pub fn from_char(ch: char) -> Self {
        if ch == '.' {
            return Self::None;
        }
        if ch.is_digit(10) {
            return Self::Value;
        }

        return Self::Symbol;
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub value: String,
    pub token_type: TokenType,
    pub x: usize,
    pub y: usize,
}

impl Token {
    pub fn extend(&self, ch: char) -> Self {
        Token {
            value: format!("{}{}", self.value, ch),
            token_type: self.token_type,
            x: self.x,
            y: self.y,
        }
    }

    pub fn new_empty() -> Self {
        Token {
            value: String::new(),
            token_type: TokenType::None,
            x: 0,
            y: 0,
        }
    }
}

pub fn parse_tokens(raw: String) -> Vec<Token> {
    let mut tokens = Vec::new();

    for (y, line) in raw.lines().enumerate() {
        let mut token: Token = Token::new_empty();

        for (x, ch) in line.chars().enumerate() {
            let current_token_type = TokenType::from_char(ch);

            if current_token_type == token.token_type {
                token = token.extend(ch);
            } else {
                if token.token_type != TokenType::None {
                    tokens.push(token);
                }
                token = Token {
                    token_type: current_token_type,
                    value: format!("{}", ch),
                    x,
                    y,
                }
            }
        }
        if token.token_type != TokenType::None {
            tokens.push(token);
        }
    }

    return tokens;
}
