#[derive(Debug, PartialEq)]
pub enum Error {
    Fmt(std::fmt::Error),
}

pub fn solve(input: &str) -> Result<String, Error> {
    use std::fmt::Write;

    let mut output = String::new();
    for line in input.lines() {
        let mut tokens = line
            .split(|c| matches!(c, ' ' | ',' | ';'))
            .filter(|s| !s.is_empty());
        let ty = tokens.next().unwrap();
        for token in tokens {
            write!(output, "{}", ty)?;

            let mut chars = token.chars().rev();
            while let Some(c) = chars.next() {
                match c {
                    '&' | '*' => write!(output, "{}", c)?,
                    ']' => {
                        write!(output, "[]")?;
                        chars.next();
                    }
                    _ => break,
                }
            }

            write!(output, " ")?;

            for c in token.chars() {
                match c {
                    'a'..='z' | 'A'..='Z' => write!(output, "{}", c)?,
                    _ => break,
                }
            }

            writeln!(output, ";")?;
        }
    }

    Ok(output)
}

impl From<std::fmt::Error> for Error {
    fn from(error: std::fmt::Error) -> Self {
        Error::Fmt(error)
    }
}
