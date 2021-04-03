#[derive(PartialEq, Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub kind: TokenType,
    pub start: usize,
    pub end: usize,
}

impl Token {
    pub const DEFAULT_TOKEN: Token = Token {
        kind: TokenType::Eof,
        start: 0,
        end: 0,
    };
}

#[derive(Debug)]
pub struct Lexer<'a> {
    pub source: String,
    iter: std::iter::Peekable<std::str::Chars<'a>>,
    start: usize,
    end: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &String) -> Lexer {
        Lexer {
            source: s.clone(),
            iter: s.chars().into_iter().peekable(),
            start: 0,
            end: 0,
        }
    }

    fn make_token(&mut self, t: TokenType) -> Option<Token> {
        let token = Token {
            kind: t,
            start: self.start,
            end: self.end,
        };
        return Some(token);
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if let Some(i) = self.iter.next() {
            self.start = self.end;
            self.end += 1;
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
                    while let Some(j) = self.iter.peek() {
                        if !j.is_numeric() && *j != '.' {
                            break;
                        }
                        self.iter.next();
                        self.end += 1;
                    }

                    self.make_token(TokenType::Number)
                }
                ' ' | '\n' | '\r' => {
                    while let Some(j) = self.iter.peek() {
                        if !j.is_whitespace() {
                            break;
                        }
                        self.iter.next();
                        self.end += 1;
                    }
                    self.next_token()
                }
                _ => {
                    eprintln!("Unexpected char '{}'!", i);
                    None
                }
            }
        } else {
            self.make_token(TokenType::Eof)
        }
    }

    #[allow(dead_code)]
    pub fn parse(&mut self) -> Result<bool, String> {
        while let Some(i) = self.next_token() {
            if i.kind == TokenType::Eof {
                break;
            }
        }
        return Ok(true);
    }
}
