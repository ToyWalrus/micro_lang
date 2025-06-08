#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Identifier(String),
    Plus,
    Minus,
    Divide,
    Multiply,
    Assign,
    LParen,
    RParen,
    Semi,
    EoF,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let chars: Vec<char> = input.chars().collect();
        let current_char = chars.get(0).copied();

        Lexer {
            input: chars,
            position: 0,
            current_char,
        }
    }

    fn advance(&mut self) -> bool {
        if self.position >= self.input.len() {
            self.current_char = None;
            return false;
        }

        self.position += 1;
        self.current_char = self.input.get(self.position).copied();
        true
    }

    fn skip_whitespace(&mut self) {
        while !self.current_char.is_none() && char::is_whitespace(self.current_char.unwrap()) {
            self.advance();
        }
    }

    fn read_number(&mut self) -> f64 {
        let mut num_string = String::new();
        let mut encountered_decimal = false;

        let result = loop {
            if char::is_numeric(self.current_char.unwrap_or(' ')) {
                num_string.push(self.current_char.unwrap());
            } else if self.current_char.unwrap_or_default() == '.' {
                if encountered_decimal {
                    break num_string;
                }

                encountered_decimal = true;
                num_string.push('.')
            } else {
                break num_string;
            }

            self.advance();
        };

        if result.len() == 0 {
            panic!(
                "Malformed NUMBER, cannot read at index {index} for string \"{string}\"! ({chr})",
                index = self.position,
                string = String::from_iter(&self.input),
                chr = self.current_char.unwrap_or('?')
            );
        }

        result.parse::<f64>().unwrap_or(0.0)
    }

    fn read_identifier(&mut self) -> String {
        let mut ident_string = String::new();
        let mut first_iter = true;

        let result = loop {
            let valid = match self.current_char {
                Some(x) if char::is_alphabetic(x) => true,
                Some(x) if char::is_numeric(x) && !first_iter => true,
                Some('_') => true,
                _ => false,
            };

            if valid {
                ident_string.push(self.current_char.unwrap());
            } else {
                break ident_string;
            }

            first_iter = false;
            self.advance();
        };

        if result.len() == 0 {
            panic!(
                "Malformed IDENTIFIER, cannot read at index {index} for string \"{string}\"! ({chr})",
                index = self.position,
                string = String::from_iter(&self.input),
                chr = self.current_char.unwrap_or('?')
            );
        }

        result
    }

    fn match_plain_token(&self, token_char: char) -> Result<Token, &'static str> {
        match token_char {
            '+' => Ok(Token::Plus),
            '-' => Ok(Token::Minus),
            '*' => Ok(Token::Multiply),
            '/' => Ok(Token::Divide),
            '=' => Ok(Token::Assign),
            '(' => Ok(Token::LParen),
            ')' => Ok(Token::RParen),
            ';' => Ok(Token::Semi),
            _ => Err("unknown token type"),
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        match self.current_char {
            Some(x) if self.match_plain_token(x).is_ok() => {
                let token = self.match_plain_token(x).unwrap();
                self.advance();
                token
            }
            Some(x) if char::is_numeric(x) || x == '.' => Token::Number(self.read_number()),
            Some(x) if char::is_alphanumeric(x) || x == '_' => {
                Token::Identifier(self.read_identifier())
            }
            None | _ => Token::EoF,
        }
    }
}
