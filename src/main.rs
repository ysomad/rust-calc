use std::{fmt, io, num::ParseFloatError};

#[derive(Debug)]
enum Token {
    Plus,
    Minus,
    Mul,
    Div,
    LParen,
    RParen,
    Num(f64),
}

impl Token {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '+' => Some(Self::Plus),
            '-' => Some(Self::Minus),
            '*' => Some(Self::Mul),
            '/' => Some(Self::Div),
            '(' => Some(Self::LParen),
            ')' => Some(Self::RParen),
            _ => None,
        }
    }

    fn can_start_expr(&self) -> bool {
        matches!(self, Token::Num(_) | Token::LParen)
    }

    fn is_operator(&self) -> bool {
        matches!(self, Token::Plus | Token::Minus | Token::Mul | Token::Div)
    }
}

#[derive(Debug)]
struct Expression {
    tokens: Vec<Token>,
}

impl Expression {
    fn new(raw: String) -> Result<Self, Error> {
        let tokens = Self::tokenize(raw)?;
        Self::validate_tokens(&tokens)?;
        Ok(Self { tokens: tokens })
    }

    fn tokenize(raw: String) -> Result<Vec<Token>, Error> {
        let raw = raw.replace(' ', "");

        let mut tokens: Vec<Token> = Vec::new();
        let mut num = String::new();

        for char in raw.chars() {
            if char.is_ascii_digit() || char == '.' {
                num.push(char);
                continue;
            }

            if !num.is_empty() {
                let n = num.parse::<f64>()?;
                tokens.push(Token::Num(n));
                num.clear();
            }

            match Token::from_char(char) {
                Some(t) => tokens.push(t),
                None => return Err(Error::new(format!("unknown character: {char}"))),
            }
        }

        // flush last number if string ends with a number
        if !num.is_empty() {
            let n = num.parse::<f64>()?;
            tokens.push(Token::Num(n));
        }

        Ok(tokens)
    }

    fn validate_tokens(tokens: &Vec<Token>) -> Result<(), Error> {
        const MIN_TOKENS: usize = 3;

        if tokens.len() < MIN_TOKENS {
            return Err(Error::new(format!(
                "expression must have at least {MIN_TOKENS} tokens"
            )));
        }

        if let Some(first) = tokens.first() {
            if first.is_operator() {
                return Err(Error::new("expression cannot start with an operator"));
            }
        }

        let mut parens: Vec<usize> = Vec::new();

        for (i, tok) in tokens.iter().enumerate() {
            // calculate parens
            match tok {
                Token::LParen => parens.push(i),
                Token::RParen => {
                    if parens.pop().is_none() {
                        return Err(Error::new(format!("unexpected closing paren at {i}")));
                    }
                }
                _ => {}
            }

            // validate ordering
            let next_tok = tokens.get(i + 1);

            match (tok, next_tok) {
                (tok, Some(next_tok)) if tok.is_operator() && next_tok.is_operator() => {
                    return Err(Error::new(format!("two operators in a row at {i}")));
                }
                (Token::Num(_), Some(Token::Num(_))) => {
                    return Err(Error::new(format!("two numbers in a row at {i}")));
                }
                (tok, None) if tok.is_operator() => {
                    return Err(Error::new("expression cannot end with an operator"));
                }
                _ => {}
            }
        }

        if let Some(i) = parens.first() {
            return Err(Error::new(format!("unclosed paren at {i}")));
        }

        Ok(())
    }

    fn eval(&self) -> Result<f64, Error> {
        return Err(Error::new("EVAL ERROR"));
    }
}

#[derive(Debug)]
struct Error {
    msg: String,
}

impl Error {
    fn new(msg: impl Into<String>) -> Self {
        Error { msg: msg.into() }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl From<ParseFloatError> for Error {
    fn from(e: ParseFloatError) -> Self {
        Error::new(e.to_string())
    }
}

fn main() {
    print!("> ");
    let mut raw_expr = String::new();

    io::stdin()
        .read_line(&mut raw_expr)
        .expect("Unable to read expression");

    let expr = Expression::new(raw_expr).unwrap();
    let res = expr.eval().unwrap();

    print!(" = {res}");
}
