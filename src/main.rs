use pulldown_cmark::Parser;
use std::io::{Read, stdin};

fn main() {
    // let args = env::args().collect();

    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).expect("EOF");

    let parser = Parser::new(&buffer);
    for event in parser {
        println!("{:?}", event);
    }
}
