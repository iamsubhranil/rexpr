use crate::builtin::Builtin;
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
            let argvalues = args.iter().map(|a| eval(a)).collect::<Vec<f64>>();
            match func {
                TokenType::Builtin(idx) => Builtin::exec_builtin(*idx, &argvalues),
                _ => panic!("Invalid function {:?}!", func),
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::eval;
    use crate::parser::Parser;

    pub fn expect_eq(s: &str, val: f64) {
        assert_eq!(eval(&Parser::new(s).parse()), val);
    }

    #[test]
    fn expressions() {
        expect_eq("1 + 2 * 3 / 4", 2.5);
        expect_eq("2 ^ 2 ^ 3", 256.0);
        expect_eq("1 + 2 - 3 * 4 / 5 ^ 6", 2.999232);
        expect_eq("(((((1 + 2) * 3) - 4) / 5) ^ 6)", 1.0);
    }
}
