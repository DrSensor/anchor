mod parser;

use std::io::stdin;

use parser::AnchorMarkdown;

fn main() {
    // let args = env::args().collect();
    let parser = AnchorMarkdown::parse_ext(stdin(), "yay -S").unwrap();
    println!("{}\n", parser.get_prepare_instruction().unwrap());
    println!("{}", parser.get_install_instruction().unwrap());
}
