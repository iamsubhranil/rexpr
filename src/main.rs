mod eval;
mod lexer;
mod parser;

fn main() -> std::io::Result<()> {
    println!("Enter the expression: ");
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;
    println!("Expression read: {}", line);
    let mut parser = parser::Parser::new(&line);
    let tree = parser.parse();
    println!("Parsed tree: {:?}", tree);
    println!("Result: {}", eval::eval(&tree));
    Ok(())
}
