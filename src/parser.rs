use crate::lexer::{Lexer, Token, TokenType};

#[derive(Debug)]
pub enum Node {
    Literal(f64),
    Function(TokenType, Vec<Box<Node>>),
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

pub struct Parser<'a> {
    lex: Lexer<'a>,
    current: Token,
    next: Token,
}

impl<'a> Parser<'a> {
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

    fn current_string(&self) -> String {
        self.lex.source[self.current.start..self.current.end].to_string()
    }

    fn expect(&mut self, t: TokenType, err: &str) {
        if self.current.kind != t {
            panic!(
                "{}, Received: '{}' ({:?})!",
                err,
                self.current_string(),
                self.current.kind
            );
        }
        self.advance();
    }

    fn try_concat<F>(&mut self, concat: F, types: &[TokenType]) -> Node
    where
        F: Fn(&'_ mut Parser<'a>) -> Node,
    {
        let mut left = concat(self);
        loop {
            match self.peek_type(types) {
                None => break,
                Some(s) => {
                    let right = concat(self);
                    left = Node::Operator(Box::from(left), s.kind, Box::from(right));
                }
            }
        }
        return left;
    }

    fn parse_term(&mut self) -> Node {
        match self.current.kind {
            TokenType::Number => {
                let oldstart = self.current.start;
                let s = self.current_string();
                self.advance();
                Node::Literal(s.parse::<f64>().unwrap_or_else(|_| {
                    panic!("Invalid decimal number '{}' at pos {}!", s, oldstart);
                }))
            }
            TokenType::ParenOpen => {
                self.advance();
                let s = self.parse_expr();
                self.expect(TokenType::ParenClose, "Expected ')'!");
                s
            }
            _ if self.current.kind.is_keyword() => {
                let t = self.current.kind;
                self.advance();
                self.expect(TokenType::ParenOpen, "Expected '(' after function call!");
                let n = t.arg_count();
                let mut s = vec![];
                if n > 0 {
                    s.push(Box::from(self.parse_expr()));
                    for _ in 1..n {
                        self.expect(TokenType::Comma, "Expected ',' after argument!");
                        s.push(Box::from(self.parse_expr()));
                    }
                }
                self.expect(TokenType::ParenClose, "Expected ')' after function call!");
                Node::Function(t, s)
            }
            _ => {
                let s = self.current_string();
                panic!(
                    "Unexpected token '{}'({:?}) at pos {}",
                    s, self.current.kind, self.current.start
                );
            }
        }
    }

    fn parse_exp(&mut self) -> Node {
        return self.try_concat(Parser::parse_term, &[TokenType::Cap]);
    }

    fn parse_fact(&mut self) -> Node {
        return self.try_concat(Parser::parse_exp, &[TokenType::Star, TokenType::Backslash]);
    }

    fn parse_sum(&mut self) -> Node {
        return self.try_concat(Parser::parse_fact, &[TokenType::Plus, TokenType::Minus]);
    }

    fn parse_mod(&mut self) -> Node {
        return self.try_concat(Parser::parse_sum, &[TokenType::Percentage]);
    }

    fn parse_expr(&mut self) -> Node {
        return self.parse_mod();
    }

    pub fn parse(&mut self) -> Node {
        self.advance(); // init current and next
        self.advance();
        return self.parse_expr();
    }
}
