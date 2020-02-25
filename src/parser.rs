use pulldown_cmark::{Parser, Tag};
use std::io::{self, Read, Write};
use std::str::{from_utf8, Utf8Error};

#[derive(Default)]
pub struct AnchorMarkdown<O: Write> {
    pub install: O,
    pub prepare: O,
}

enum State {
    Category(String),
    Distro(String),
    NaN,
}

impl AnchorMarkdown<Vec<u8>> {
    pub fn get_install_instruction(&self) -> Result<&str, Utf8Error> {
        from_utf8(&self.install)
    }
    pub fn get_prepare_instruction(&self) -> Result<&str, Utf8Error> {
        from_utf8(&self.prepare)
    }
}

impl<O> AnchorMarkdown<O>
where
    O: Write + Default, // TODO: remove `Default` requirement
                        //       so it can work directly on std::fs::File
{
    // I use Read instead of String to support future use-cases
    pub fn parse_ext<I: Read>(mut input: I, install_cmd: &str) -> io::Result<Self> {
        let mut md = Self::default();

        let mut buffer = String::new();
        input.read_to_string(&mut buffer)?;
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
                                md.install,
                                "RUN {} {}",
                                install_cmd.trim(),
                                value.sanitize()
                            )?;
                            write!(md.install, "\nLABEL package={}\n\n", &key.to_lowercase())?;
                        }
                        (Fenced(ext), Distro(_key)) if ext.as_ref() == "sh" => {
                            write!(md.prepare, "RUN {}", value.sanitize())?;
                        }
                        _ => continue,
                    },
                    _ => continue,
                },
                _ => continue,
            };
        }

        Ok(md)
    }
}

trait Sanitize<T: ToString> {
    fn sanitize(self) -> T;
}

impl Sanitize<String> for &str {
    fn sanitize(self) -> String {
        self.replace("\n", " \\\n    ")
            .trim_end_matches(" \\\n    ")
            .trim()
            .to_string()
    }
}
