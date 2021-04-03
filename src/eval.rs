use crate::lexer::TokenType;
use crate::parser::Node;

pub fn eval(t: &Node) -> f64 {
    match t {
        Node::Literal(s) => *s,
        Node::Operator(l, o, r) => {
            let a = eval(l.as_ref());
            let b = eval(r.as_ref());
            match o {
                TokenType::Plus => a + b,
                TokenType::Minus => a - b,
                TokenType::Star => a * b,
                TokenType::Backslash => a / b,
                TokenType::Cap => a.powf(b),
                TokenType::Percentage => a % b,
                _ => panic!("Invalid operator {:?}!", o),
            }
        }
    }
}
