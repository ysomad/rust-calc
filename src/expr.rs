use crate::error::Error;
use crate::token::Token;

#[derive(Debug)]
pub struct Expression {
    tokens: Vec<Token>,
}

impl Expression {
    pub fn new(raw: &str) -> Result<Self, Error> {
        let tokens = Self::tokenize(raw)?;
        Self::validate_tokens(&tokens)?;
        Ok(Self { tokens: tokens })
    }

    fn tokenize(raw: &str) -> Result<Vec<Token>, Error> {
        let raw = raw.trim().replace(' ', "");

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

    fn rpn_output(&self) -> Vec<Token> {
        let mut output: Vec<Token> = Vec::new();
        let mut ops: Vec<Token> = Vec::new();

        for tok in &self.tokens {
            match tok {
                Token::Num(_) => output.push(*tok),
                Token::LParen => ops.push(*tok),
                Token::RParen => {
                    while let Some(op) = ops.pop() {
                        if op == Token::LParen {
                            break;
                        }
                        output.push(op);
                    }
                }
                _ => {
                    while let Some(last) = ops.last() {
                        if *last == Token::LParen {
                            break;
                        }
                        if last.precedence() < tok.precedence() {
                            break;
                        }
                        if last.precedence() == tok.precedence() && !tok.is_left_associative() {
                            break;
                        }
                        output.push(ops.pop().unwrap());
                    }
                    ops.push(*tok);
                }
            }
        }

        while let Some(op) = ops.pop() {
            output.push(op);
        }

        output
    }

    // evaluates expression using Shunting-Yard Algorithm
    // https://en.wikipedia.org/wiki/Shunting_yard_algorithm
    // evaluates expression using Shunting-Yard Algorithm
    // https://en.wikipedia.org/wiki/Shunting_yard_algorithm
    pub fn eval(&self) -> Result<f64, Error> {
        let output = self.rpn_output();
        let mut nums: Vec<f64> = Vec::new();

        for tok in &output {
            match tok {
                Token::Num(n) => nums.push(*n),
                _ => {
                    let b = nums.pop().ok_or(Error::new("b not found"))?;
                    let a = nums.pop().ok_or(Error::new("a not found"))?;
                    let res = match tok {
                        Token::Plus => a + b,
                        Token::Minus => a - b,
                        Token::Mul => a * b,
                        Token::Div => a / b,
                        Token::Pow => a.powf(b),
                        _ => return Err(Error::new("unknown token in evaluation")),
                    };

                    nums.push(res);
                }
            }
        }

        nums.pop().ok_or(Error::new("invalid expr"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eval(input: &str) -> f64 {
        Expression::new(input).unwrap().eval().unwrap()
    }

    #[test]
    fn addition() {
        assert_eq!(eval("1+2"), 3.0);
    }

    #[test]
    fn subtraction() {
        assert_eq!(eval("10-3"), 7.0);
    }

    #[test]
    fn multiplication() {
        assert_eq!(eval("4*5"), 20.0);
    }

    #[test]
    fn division() {
        assert_eq!(eval("20/4"), 5.0);
    }

    #[test]
    fn exponentiation() {
        assert_eq!(eval("2^3"), 8.0);
    }

    #[test]
    fn operator_precedence() {
        assert_eq!(eval("3+5*5"), 28.0);
        assert_eq!(eval("2+3*4+5"), 19.0);
        assert_eq!(eval("10-2*3"), 4.0);
    }

    #[test]
    fn parentheses() {
        assert_eq!(eval("(3+5)*5"), 40.0);
        assert_eq!(eval("(2+3)*(4+5)"), 45.0);
    }

    #[test]
    fn nested_parentheses() {
        assert_eq!(eval("((2+3))*2"), 10.0);
    }

    #[test]
    fn exponent_right_associative() {
        assert_eq!(eval("2^2^3"), 256.0); // 2^(2^3) = 2^8 = 256
    }

    #[test]
    fn decimals() {
        assert_eq!(eval("1.5+2.5"), 4.0);
    }

    #[test]
    fn whitespace() {
        assert_eq!(eval(" 3 + 5 * 5 "), 28.0);
    }

    #[test]
    fn invalid_start_with_operator() {
        assert!(Expression::new("+1+2").is_err());
    }

    #[test]
    fn invalid_two_operators() {
        assert!(Expression::new("1++2").is_err());
    }

    #[test]
    fn invalid_unclosed_paren() {
        assert!(Expression::new("(1+2").is_err());
    }

    #[test]
    fn invalid_too_short() {
        assert!(Expression::new("1+").is_err());
    }
}
