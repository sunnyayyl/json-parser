use std::io::{stdin, stdout, Write};
use lexer::Lexer;
use parser::Parser;
fn main() {
    loop {
        print!(">>> ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let lexer = Lexer::new(&input);
        for token in lexer.clone(){
            println!("{}", token);
        }
        println!("{:?}", lexer.clone().into_iter().collect::<Vec<_>>());
        let mut parser=Parser::new(lexer);
        println!("{:#?}",parser.parse());
    }
}
