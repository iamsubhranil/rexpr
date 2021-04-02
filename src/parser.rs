use crate::lexer;

struct Node {
    left: NodeType,
    operator: lexer::TokenType,
    right: NodeType,
}

enum NodeType {
    Literal(f64),
    Operator(Box<Node>),
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

struct Parser {
    lex: lexer::Lexer,
}

impl Parser {
    fn new(l: lexer::Lexer) -> Parser {
        Parser { lex: l }
    }

    fn advance(&self) -> &lexer::Token {
        return self.lex.next_token().unwrap();
    }

    fn parse_mod(&self, t: &lexer::Token) -> NodeType {
        let t = self.parse_sum(t);
    }

    fn parse_expr(&self, t: &lexer::Token) -> NodeType {
        return parse_mod(t);
    }

    fn parse(&self) -> NodeType {
        return self.parse_expr(self.lex.next_token().unwrap());
    }
}
