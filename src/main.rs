use pulldown_cmark::{Parser, Tag};
use std::io::{stdin, stdout, Read, Write};

#[derive(Debug, PartialEq)]
enum State {
    Category(String),
    Distro(String),
    NaN,
}

fn main() {
    // let args = env::args().collect();

    let mut buffer = String::new();
    let mut output = stdout();
    let install_cmd = "yay -S";
    stdin().read_to_string(&mut buffer).expect("EOF");

    let parser = Parser::new(&buffer);
    let (mut state, mut tag) = (State::NaN, Tag::Heading(0));
    let mut inpkg = false;
    for event in parser {
        use pulldown_cmark::{CodeBlockKind::*, Event::*, Tag::*};
        use State::*;

        match event.clone() {
            Start(_t @ Heading(0..=1)) if inpkg => inpkg = false,
            Start(t) => tag = t,
            Text(string) => match (&tag, string.as_ref()) {
                (_t @ Heading(2), "Packages") => inpkg = true,
                (_t @ Heading(3), key) if inpkg => state = Category(key.into()),
                (_t @ Heading(4), key) if inpkg => state = Distro(key.into()),

                (CodeBlock(kind), value) => match (kind, &state) {
                    (_, Category(key)) => {
                        write!(
                            output,
                            "RUN {} {}",
                            install_cmd.trim(),
                            &value
                                .replace("\n", " \\\n    ")
                                .trim_end_matches(" \\\n    ")
                                .trim()
                        )
                        .expect("utf-8");
                        write!(stdout(), "\nLABEL package={}\n\n", &key.to_lowercase())
                            .expect("utf-8");
                    }
                    (Fenced(ext), Distro(_key)) if ext.as_ref() == "sh" => {
                        write!(
                            output,
                            "RUN {}",
                            &value
                                .replace("\n", " \\\n    ")
                                .trim_end_matches(" \\\n    ")
                                .trim()
                        )
                        .expect("utf-8");
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        };
    }
}
