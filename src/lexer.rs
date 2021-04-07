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
    Comma,

    KeywordSin,
    KeywordCos,
    KeywordTan,
    KeywordDegrees,
    KeywordRadians,
    KeywordAbs,

    Eof,
}

impl TokenType {
    pub fn is_keyword(&self) -> bool {
        match self {
            TokenType::KeywordSin
            | TokenType::KeywordCos
            | TokenType::KeywordTan
            | TokenType::KeywordDegrees
            | TokenType::KeywordRadians
            | TokenType::KeywordAbs => true,
            _ => false,
        }
    }

    pub fn arg_count(&self) -> i32 {
        match self {
            TokenType::KeywordSin
            | TokenType::KeywordCos
            | TokenType::KeywordTan
            | TokenType::KeywordDegrees
            | TokenType::KeywordRadians
            | TokenType::KeywordAbs => 1,
            _ => 0,
        }
    }
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

    fn has_keyword(s: &str) -> Option<TokenType> {
        match s {
            "sin" => Some(TokenType::KeywordSin),
            "cos" => Some(TokenType::KeywordCos),
            "tan" => Some(TokenType::KeywordTan),
            "degrees" => Some(TokenType::KeywordDegrees),
            "radians" => Some(TokenType::KeywordRadians),
            "abs" => Some(TokenType::KeywordAbs),
            _ => None,
        }
    }

    fn make_identifier(&mut self) -> Option<Token> {
        match Lexer::has_keyword(&self.source[self.start..self.end]) {
            Some(s) => self.make_token(s),
            None => {
                eprintln!(
                    "Unexpected identifier '{}'",
                    self.source[self.start..self.end].to_string()
                );
                None
            }
        }
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
                ',' => self.make_token(TokenType::Comma),
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
                _ if i.is_alphabetic() => {
                    while let Some(j) = self.iter.peek() {
                        if !j.is_alphanumeric() && *j != '_' {
                            break;
                        }
                        self.iter.next();
                        self.end += 1;
                    }

                    self.make_identifier()
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
