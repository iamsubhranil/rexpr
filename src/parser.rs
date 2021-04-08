use crate::lexer::{Lexer, Token, TokenType};

#[derive(Debug, PartialEq)]
pub enum Node {
    Literal(f64),
    Function(TokenType, Vec<Node>),
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
    pub fn new(s: &str) -> Parser {
        Parser {
            lex: Lexer::new(s),
            current: Token::DEFAULT_TOKEN,
            next: Token::DEFAULT_TOKEN,
        }
    }

    fn advance(&mut self) -> Token {
        self.current = self.next;
        self.next = self.lex.next_token();
        self.current
    }

    fn peek_type(&mut self, t: &[TokenType]) -> Option<Token> {
        if t.contains(&self.current.kind) {
            let s = self.current;
            self.advance();
            return Some(s);
        }
        None
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
        left
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
                    s.push(self.parse_expr());
                    for _ in 1..n {
                        self.expect(TokenType::Comma, "Expected ',' after argument!");
                        s.push(self.parse_expr());
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
        let left = self.parse_term();
        match self.peek_type(&[TokenType::Cap]) {
            Some(s) => {
                let right = self.parse_exp();
                Node::Operator(Box::from(left), s.kind, Box::from(right))
            }
            None => left,
        }
    }

    fn parse_fact(&mut self) -> Node {
        self.try_concat(Parser::parse_exp, &[TokenType::Star, TokenType::Backslash])
    }

    fn parse_sum(&mut self) -> Node {
        self.try_concat(Parser::parse_fact, &[TokenType::Plus, TokenType::Minus])
    }

    fn parse_mod(&mut self) -> Node {
        self.try_concat(Parser::parse_sum, &[TokenType::Percentage])
    }

    fn parse_expr(&mut self) -> Node {
        self.parse_mod()
    }

    pub fn parse(&mut self) -> Node {
        self.advance(); // init current and next
        self.advance();
        self.parse_expr()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expect_eq(s1: &str, s2: &str) {
        let r1 = Parser::new(s1).parse();
        let r2 = Parser::new(s2).parse();
        assert_eq!(r1, r2);
    }

    fn expect_neq(s1: &str, s2: &str) {
        let r1 = Parser::new(s1).parse();
        let r2 = Parser::new(s2).parse();
        assert_ne!(r1, r2);
    }

    fn expect_err(s1: &str) {
        let res = std::panic::catch_unwind(|| Parser::new(s1).parse());
        assert!(res.is_err());
    }

    #[test]
    fn associativity() {
        expect_eq("1 + 2 + 3", "(1 + 2) + 3");
        expect_eq("1 + 2 - 3", "(1 + 2) - 3");
        expect_eq("1 * 2 / 3", "(1 * 2) / 3");
        expect_eq("1 ^ 2 ^ 3", "1 ^ (2 ^ 3)");
        expect_eq("sin(1 + 2 + 3)", "sin((1 + 2) + 3)");
    }

    #[test]
    fn precedence() {
        expect_eq("1 + 2 - 3", "(1 + 2) - 3");
        expect_neq("1 + 2 * 3", "(1 + 2) * 3");
        expect_eq("1 + 2 * 3", "1 + (2 * 3)");
        expect_neq("1 / 2 - 3", "1 / (2 - 3)");
        expect_eq("1 / 2 - 3", "(1 / 2) - 3)");
        expect_neq("1 + 2 - 3 * 4 ^ 5", "(1 + 2) - (3  * 4) ^ 5");
        expect_eq("1 + 2 - 3 * 4 ^ 5", "(1 + 2) - (3  * (4 ^ 5))");
        expect_eq("1 + 2 + sin(1000)", "((1 + 2) + sin(1000))");
    }

    #[test]
    fn invalid_input() {
        expect_err("1 +");
        expect_err("+");
        expect_err("((1 + 2)");
        expect_err("hello!");
        expect_err("sin(100, 200, 300)");
        expect_err("cos");
        expect_err("cos(100");
        expect_err("23 * 46 /");
        expect_err("1.2.3");
        expect_err("cos(100 200)");
        expect_err("sin ^ cos");
    }
}
