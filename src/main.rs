use std::any::Any;
use std::time::Instant;
use umber_lang::lexer::Lexer;
use umber_lang::parser::Parser;

static TEXT_TO_LEX: &'static str = "\

";


fn main() {
    let mut l = Lexer::new("Test", TEXT_TO_LEX);

    let start_time = Instant::now();

    let (tokens, error) = l.make_tokens();

    if error.is_some() {
        panic!("Couldnt make tokens: {}", error.unwrap());
    }

    let mut parser = Parser::new(tokens);
    let parse_res = parser.parse();

    if parse_res.has_error() {
        println!("parser err: {}!", parse_res.error().as_ref().unwrap());
        return;
    }

    println!("took: {}ms", start_time.elapsed().as_millis());
    println!("node: {}", parse_res.node().as_ref().unwrap());

}
