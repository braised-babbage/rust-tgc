pub mod ast;
pub mod intern;
pub mod lexer;
pub mod parser;

fn main() {
    let source = "
       let x = 10;
       let y = (x + 5)*2;
       print(\"result:\", y);
    ";

    let lex = lexer::Lexer::new(source);
    for tok in lex {
	print!("{:?}\n", tok);
    }
}
