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
        Node::Function(func, args) => {
            let mut argvalues = vec![];
            for a in args {
                argvalues.push(eval(a));
            }
            match func {
                TokenType::KeywordSin => argvalues[0].sin(),
                TokenType::KeywordCos => argvalues[0].cos(),
                TokenType::KeywordTan => argvalues[0].tan(),
                TokenType::KeywordDegrees => argvalues[0].to_degrees(),
                TokenType::KeywordRadians => argvalues[0].to_radians(),
                TokenType::KeywordAbs => argvalues[0].abs(),
                _ => panic!("Invalid function {:?}!", func),
            }
        }
    }
}
