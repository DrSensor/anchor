mod parser;

use std::{
    env::args,
    fs::File,
    io::{self, BufRead, BufReader},
};

use parser::AnchorMarkdown;

fn main() {
    let file = args().nth(1).unwrap_or("./Dockerfile".to_string());
    let dockerfile = File::open(file).expect("dockerfile must exists!");
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

fn transform<O: io::Write + Default>(filepath: &str, _cmd: &str) -> io::Result<AnchorMarkdown<O>> {
    let packages_md = File::open(filepath).expect("exists!");
    let reader = BufReader::new(packages_md);
    AnchorMarkdown::parse_ext(reader, "yay -S")
}
