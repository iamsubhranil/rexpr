use crate::lexer::{Lexer, Token, TokenType};

#[derive(Debug)]
pub enum Node {
    Literal(f64),
    Operator(Box<Node>, TokenType, Box<Node>),
}

/*
 * Grammar
 * Expr -> Mod
 * Mod -> Sum [ "%" Sum ] *
 * Sum -> Fact [ ( "+" | "-" ) Fact ]*
 * Fact -> Exp [ ( "*" | "/" ) Exp ] *
 * Exp -> Term [ ( "^" ) Term ] *
 * Term -> Number | Group
 * Group -> "(" Expr ")"
 *
 */

pub struct Parser {
    lex: Lexer,
    current: Token,
    next: Token,
}

impl Parser {
    pub fn new(s: &String) -> Parser {
        Parser {
            lex: Lexer::new(s),
            current: Token::DEFAULT_TOKEN,
            next: Token::DEFAULT_TOKEN,
        }
    }

    fn advance(&mut self) -> Token {
        self.current = self.next;
        self.next = self.lex.next_token().unwrap();
        return self.current;
    }

    fn peek_type(&mut self, t: &[TokenType]) -> Option<Token> {
        if t.contains(&self.current.kind) {
            let s = self.current;
            self.advance();
            return Some(s);
        }
        return None;
    }

    fn expect(&mut self, t: TokenType, err: &str) {
        if self.current.kind != t {
            panic!("{}", err);
        }
        self.advance();
    }

    fn parse_term(&mut self) -> Node {
        match self.current.kind {
            TokenType::Number => {
                let s: String = self.lex.source[self.current.start..self.current.end]
                    .iter()
                    .collect();
                self.advance();
                Node::Literal(s.parse::<f64>().expect("Expected valid number!"))
            }
            TokenType::ParenOpen => {
                self.advance();
                let s = self.parse_expr();
                self.expect(TokenType::ParenClose, "Expected ')'!");
                s
            }
            _ => {
                let s: String = self.lex.source[self.current.start..self.current.end]
                    .iter()
                    .collect();
                panic!("Unexpected token '{}' at pos {}", s, self.current.start);
            }
        }
    }

    fn parse_exp(&mut self) -> Node {
        let mut t = self.parse_term();
        loop {
            match self.peek_type(&[TokenType::Cap]) {
                None => break,
                Some(s) => {
                    let r = self.parse_term();
                    t = Node::Operator(Box::from(t), s.kind, Box::from(r));
                }
            }
        }
        return t;
    }

    fn parse_fact(&mut self) -> Node {
        let mut t = self.parse_exp();
        loop {
            match self.peek_type(&[TokenType::Star, TokenType::Backslash]) {
                None => break,
                Some(s) => {
                    let r = self.parse_exp();
                    t = Node::Operator(Box::from(t), s.kind, Box::from(r));
                }
            }
        }
        return t;
    }

    fn parse_sum(&mut self) -> Node {
        let mut t = self.parse_fact();
        loop {
            match self.peek_type(&[TokenType::Plus, TokenType::Minus]) {
                None => break,
                Some(s) => {
                    let r = self.parse_fact();
                    t = Node::Operator(Box::from(t), s.kind, Box::from(r));
                }
            }
        }
        return t;
    }

    fn parse_mod(&mut self) -> Node {
        let mut t = self.parse_sum();
        loop {
            match self.peek_type(&[TokenType::Percentage]) {
                None => break,
                Some(s) => {
                    let r = self.parse_sum();
                    t = Node::Operator(Box::from(t), s.kind, Box::from(r));
                }
            }
        }
        return t;
    }

    fn parse_expr(&mut self) -> Node {
        return self.parse_mod();
    }

    pub fn parse(&mut self) -> Node {
        self.advance();
        self.advance();
        return self.parse_expr();
    }
}
