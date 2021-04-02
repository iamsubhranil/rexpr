mod lexer;
//mod parser;

fn main() -> std::io::Result<()> {
    println!("Enter the expression: ");
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;
    println!("Expression read: {}", line);
    let mut lexer = lexer::Lexer::new(&line);
    match lexer.parse() {
        Ok(_) => println!("{:?}", lexer),
        Err(x) => panic!("{}", x),
    }
    Ok(())
}
