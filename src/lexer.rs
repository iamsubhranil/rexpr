#[derive(PartialEq, Debug)]
pub enum TokenType {
    Number,
    Plus,
    Minus,
    Star,
    Backslash,
    Cap,
    Percentage,
    ParenOpen,
    ParenClose,
    Eof,
}

#[derive(Debug)]
pub struct Token {
    kind: TokenType,
    start: usize,
    length: usize,
}

#[derive(Debug)]
pub struct Lexer {
    source: Vec<char>,
    pub tokens: Vec<Token>,
    start: usize,
    end: usize,
    total: usize,
}

impl Lexer {
    pub fn new(s: &String) -> Lexer {
        Lexer {
            source: s.chars().collect(),
            tokens: vec![],
            start: 0,
            end: 0,
            total: s.len(),
        }
    }

    fn make_token(&mut self, t: TokenType) -> Option<&Token> {
        let token = Token {
            kind: t,
            start: self.start,
            length: self.end - self.start,
        };
        self.tokens.push(token);
        self.tokens.last()
    }

    pub fn next_token(&mut self) -> Option<&Token> {
        if self.end >= self.total {
            return self.make_token(TokenType::Eof);
        }
        self.start = self.end;
        self.end += 1;
        let i = self.source[self.start];
        match i {
            '+' => self.make_token(TokenType::Plus),
            '-' => self.make_token(TokenType::Minus),
            '*' => self.make_token(TokenType::Star),
            '/' => self.make_token(TokenType::Backslash),
            '^' => self.make_token(TokenType::Cap),
            '%' => self.make_token(TokenType::Percentage),
            '(' => self.make_token(TokenType::ParenOpen),
            ')' => self.make_token(TokenType::ParenClose),
            _ if i.is_numeric() => {
                while self.source[self.end].is_numeric() {
                    self.end += 1
                }
                self.make_token(TokenType::Number)
            }
            ' ' | '\n' | '\r' => {
                while self.source[self.end].is_whitespace() {
                    self.end += 1;
                }
                self.next_token()
            }
            _ => {
                eprintln!("Unexpected char '{}'!", i);
                None
            }
        }
    }

    pub fn parse(&mut self) -> Result<bool, String> {
        while self.end < self.total - 1 {
            match self.next_token() {
                Some(_) => (),
                None => return Err("Error occurred while scanning!".to_string()),
            }
        }
        return Ok(true);
    }
}
