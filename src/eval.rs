use crate::lexer::TokenType;
use crate::parser::Node;

pub fn eval(tree: &Node) -> f64 {
    match tree {
        Node::Literal(val) => *val,
        Node::Operator(left, operator, right) => {
            let leftval = eval(left.as_ref());
            let rightval = eval(right.as_ref());
            match operator {
                TokenType::Plus => leftval + rightval,
                TokenType::Minus => leftval - rightval,
                TokenType::Star => leftval * rightval,
                TokenType::Backslash => leftval / rightval,
                TokenType::Cap => leftval.powf(rightval),
                TokenType::Percentage => leftval % rightval,
                _ => panic!("Invalid operator {:?}!", operator),
            }
        }
    }
}
