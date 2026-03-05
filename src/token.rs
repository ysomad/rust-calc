#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Plus,
    Minus,
    Mul,
    Div,
    Pow,
    LParen,
    RParen,
    Num(f64),
}

impl Token {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '+' => Some(Self::Plus),
            '-' => Some(Self::Minus),
            '*' => Some(Self::Mul),
            '/' => Some(Self::Div),
            '^' => Some(Self::Pow),
            '(' => Some(Self::LParen),
            ')' => Some(Self::RParen),
            _ => None,
        }
    }

    pub fn is_operator(&self) -> bool {
        matches!(
            self,
            Self::Plus | Self::Minus | Self::Mul | Self::Div | Self::Pow
        )
    }

    // https://en.wikipedia.org/wiki/Order_of_operations
    pub fn precedence(&self) -> u8 {
        match self {
            Self::Plus | Self::Minus => 1,
            Self::Mul | Self::Div => 2,
            Self::Pow => 3,
            _ => 0,
        }
    }

    pub fn is_left_associative(&self) -> bool {
        match self {
            Token::Plus | Token::Minus | Token::Mul | Token::Div => true,
            _ => false,
        }
    }
}
