use crate::builtin::Builtin;

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

    Builtin(usize),

    Error,
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
    pub fn new(s: &str) -> Lexer {
        Lexer {
            source: s.to_string(),
            iter: s.chars().into_iter().peekable(),
            start: 0,
            end: 0,
        }
    }

    fn make_token(&mut self, t: TokenType) -> Token {
        Token {
            kind: t,
            start: self.start,
            end: self.end,
        }
    }

    fn has_keyword(s: &str) -> Option<TokenType> {
        let f = Builtin::has_builtin(s);
        match f {
            Some(s) => Some(TokenType::Builtin(s)),
            None => None,
        }
    }

    fn make_identifier(&mut self) -> Token {
        match Lexer::has_keyword(&self.source[self.start..self.end]) {
            Some(s) => self.make_token(s),
            None => {
                eprintln!(
                    "Unexpected identifier '{}'",
                    self.source[self.start..self.end].to_string()
                );
                self.make_token(TokenType::Error)
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
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
                    self.make_token(TokenType::Error)
                }
            }
        } else {
            self.make_token(TokenType::Eof)
        }
    }

    #[allow(dead_code)]
    pub fn parse(&mut self) -> Result<Vec<Token>, String> {
        let mut l = vec![];
        loop {
            let i = self.next_token();
            l.push(i);
            if i.kind == TokenType::Eof {
                break;
            }
        }
        Ok(l)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn check_parse(source: &str, mut types: Vec<TokenType>) {
        let mut l = Lexer::new(&source);
        let tokens = l.parse().unwrap();
        types.push(TokenType::Eof);
        let f = tokens.iter().zip(types.iter());
        for it in f {
            let (a, b) = it;
            assert_eq!(a.kind, *b);
        }
    }

    #[test]
    fn number() {
        check_parse(
            "1 1.23 0.1231 99388128 1.2.21",
            vec![
                TokenType::Number,
                TokenType::Number,
                TokenType::Number,
                TokenType::Number,
                TokenType::Number,
            ],
        );
    }

    #[test]
    fn operators_and_tokens() {
        check_parse(
            "(+-*/^)%,",
            vec![
                TokenType::ParenOpen,
                TokenType::Plus,
                TokenType::Minus,
                TokenType::Star,
                TokenType::Backslash,
                TokenType::Cap,
                TokenType::ParenClose,
                TokenType::Percentage,
                TokenType::Comma,
            ],
        );
    }
}
