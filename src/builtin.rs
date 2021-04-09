pub struct Builtin {
    name: &'static str,
    args: usize,
    eval: fn(&[f64]) -> f64,
}

macro_rules! b {
    ($n:ident, $a:expr, $e:expr) => {
        Builtin {
            name: stringify!($n),
            args: $a,
            eval: $e,
        }
    };
}

macro_rules! b1 {
    ($n:ident) => {
        b!($n, 1, |args| args[0].$n())
    };
    ($n:ident, $e:expr) => {
        b!($n, 1, $e)
    };
}

impl Builtin {
    const BUILTINS: [Builtin; 15] = [
        b1!(sin),
        b1!(cos),
        b1!(tan),
        b1!(asin),
        b1!(acos),
        b1!(atan),
        b1!(sinh),
        b1!(cosh),
        b1!(tanh),
        b1!(asinh),
        b1!(acosh),
        b1!(atanh),
        b1!(abs),
        b1!(deg, |args| args[0].to_degrees()),
        b1!(rad, |args| args[0].to_radians()),
    ];

    pub fn has_builtin(s: &str) -> Option<usize> {
        let f = Builtin::BUILTINS
            .iter()
            .enumerate()
            .find(|a| a.1.name.eq(s));
        match f {
            Some(b) => Some(b.0),
            None => None,
        }
    }

    pub fn arg_count(b: usize) -> usize {
        Builtin::BUILTINS[b].args
    }

    pub fn exec_builtin(b: usize, args: &[f64]) -> f64 {
        (Builtin::BUILTINS[b].eval)(args)
    }
}

#[cfg(test)]
mod tests {
    use crate::eval::tests::expect_eq as eval_expect_eq;
    use crate::lexer::tests::check_parse as lexer_check_parse;
    use crate::lexer::TokenType;
    use crate::parser::tests::expect_eq as parser_expect_eq;
    use crate::parser::tests::expect_err as parser_expect_err;
    use crate::parser::tests::expect_neq as parser_expect_neq;

    #[test]
    fn builtin_lex() {
        let mut v = vec![];
        for i in 0..15 {
            v.push(TokenType::Builtin(i));
        }
        lexer_check_parse(
            "sin cos tan asin acos atan sinh cosh tanh asinh acosh atanh abs deg rad",
            v,
        );
    }

    #[test]
    fn builtin_lex_err() {
        lexer_check_parse(
            "sincos tanacos degrad absabs",
            vec![
                TokenType::Error,
                TokenType::Error,
                TokenType::Error,
                TokenType::Error,
            ],
        );
    }

    #[test]
    fn builtin_parse() {
        parser_expect_eq(
            "sin(100) * cos(100) + tan(102.21)",
            "(sin(100) * cos(100)) + tan(102.21)",
        );
        parser_expect_eq(
            "sin(cos(1 + 2 * abs(deg(200))))",
            "sin(cos(1 + (2 * abs(deg(200)))))",
        );
        parser_expect_neq(
            "sin(cos(1 + 2 * abs(deg(200))))",
            "sin(cos((1 + 2) * abs(deg(200))))",
        );
    }

    #[test]
    fn builtin_parse_err() {
        parser_expect_err("sin(1, 2)");
        parser_expect_err("cos(1, 2)");
        parser_expect_err("tan(1, 2)");
        parser_expect_err("asin(1, 2)");
        parser_expect_err("acos(1, 2)");
        parser_expect_err("atan(1, 2)");
        parser_expect_err("sinh(1, 2)");
        parser_expect_err("cosh(1, 2)");
        parser_expect_err("tanh(1, 2)");
        parser_expect_err("asinh(1, 2)");
        parser_expect_err("acosh(1, 2)");
        parser_expect_err("atanh(1, 2)");
        parser_expect_err("abs(1, 2)");
        parser_expect_err("deg(1, 2)");
        parser_expect_err("rad(1, 2)");
    }

    #[test]
    fn builtin_eval() {
        eval_expect_eq("sin(90)", 0.8939966636005579);
        eval_expect_eq("sin(deg(90))", -0.9540914674728181);
        eval_expect_eq("sin(rad(90))", 1.0);
        eval_expect_eq("cos(69)", 0.9933903797222716);
        eval_expect_eq("tan(78)", -0.5991799983411151);
        eval_expect_eq("asin(1)", 1.5707963267948966);
        eval_expect_eq("acos(0.5)", 1.0471975511965979);
        eval_expect_eq("atan(1.5)", 0.982793723247329);
        eval_expect_eq("sinh(10)", 11013.232874703393);
        eval_expect_eq("cosh(10)", 11013.232920103324);
        eval_expect_eq("tanh(15)", 0.9999999999998128);
        eval_expect_eq("asinh(10)", 2.99822295029797);
        eval_expect_eq("acosh(10)", 2.993222846126381);
        eval_expect_eq("atanh(0.5)", 0.5493061443340548);
        eval_expect_eq("abs(1234)", 1234.0);
    }
}
