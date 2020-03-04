mod parser;

use std::{
    fs::File,
    io::{self, stdin, BufRead, BufReader},
};

use parser::AnchorMarkdown;

fn main() {
    // let args = enddv::args().collect();
    // TODO: check args[1] if empty then open ./Dockerfile
    let dockerfile = File::open("./Dockerfile").expect("exists!");
    let reader = BufReader::new(dockerfile);

    for line in reader.lines() {
        if let Ok(line) = line {
            let mut words = line.split_whitespace();
            match words.next().map(str::to_uppercase).as_deref() {
                Some("PREPARE") => {
                    let parser = transform(words.next().expect("utf8"), "yay -S").unwrap();
                    println!("{}", parser.get_prepare_instruction().expect("utf8"));
                }
                Some("INSTALL") => {
                    let parser = transform(words.next().expect("utf8"), "yay -S").unwrap();
                    println!(
                        "{}",
                        parser.get_install_instruction().expect("utf8").trim_end()
                    );
                }
                _ => println!("{}", line),
            };
        }
    }
}

fn transform<O: io::Write + Default>(filepath: &str, cmd: &str) -> io::Result<AnchorMarkdown<O>> {
    let packages_md = File::open(filepath).expect("exists!");
    let reader = BufReader::new(packages_md);
    AnchorMarkdown::parse_ext(reader, "yay -S")
}
