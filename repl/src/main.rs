use lexer::Lexer;
use parser::{Parser, Deserialize};
use std::io::{stdin, stdout, Write};

fn main() {
    loop {
        print!(">>> ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let lexer = Lexer::new(&input);
        /*for token in lexer.clone() {
            println!("{}", token);
        }*/
        println!("{:?}", lexer.clone().into_iter().collect::<Vec<_>>());
        let parser = Parser::new(lexer);
        let value = parser.parse();
        //println!("{:#?}", value);
        println!("{:#?}", value.deserialize());
    }
}
