#[derive(Debug, PartialEq)]
pub enum Error {
    Fmt(std::fmt::Error),
}

pub fn solve(input: &str) -> Result<String, Error> {
    use std::fmt::Write;

    let mut output = String::new();

    'line: for line in input.lines() {
        macro_rules! error {
            () => {{
                writeln!(output, "Error!")?;
                continue 'line;
            }};
        }

        let mut identifier = String::new();

        if line.contains(char::is_uppercase) {
            let mut chars = line.chars();
            match chars.next() {
                Some(c @ 'a'..='z') => write!(identifier, "{}", c)?,
                _ => error!(),
            }
            for c in chars {
                match c {
                    'A'..='Z' => write!(identifier, "_{}", c.to_lowercase())?,
                    'a'..='z' => write!(identifier, "{}", c)?,
                    _ => error!(),
                }
            }
        } else if line.contains('_') {
            let mut is_duplicated = false;
            let mut chars = line.chars();
            match chars.next() {
                Some(c @ 'a'..='z') => write!(identifier, "{}", c)?,
                _ => error!(),
            }
            for c in chars {
                match c {
                    '_' if !is_duplicated => {
                        is_duplicated = true;
                    }
                    'a'..='z' if !is_duplicated => write!(identifier, "{}", c)?,
                    'a'..='z' if is_duplicated => {
                        is_duplicated = false;
                        write!(identifier, "{}", c.to_uppercase())?;
                    }
                    _ => error!(),
                }
            }
            if is_duplicated {
                error!();
            }
        } else {
            for c in line.chars() {
                match c {
                    'a'..='z' => write!(identifier, "{}", c)?,
                    _ => error!(),
                }
            }
        }

        writeln!(output, "{}", identifier)?;
    }

    Ok(output)
}

impl From<std::fmt::Error> for Error {
    fn from(error: std::fmt::Error) -> Self {
        Error::Fmt(error)
    }
}
