fn main() {
    use std::io::Read;

    let mut io = std::io::stdin();
    let mut input = String::new();
    match io.read_to_string(&mut input) {
        Ok(_) => match solve(&input) {
            Ok(output) => print!("{}", output),
            Err(error) => print!("{:#?}", error),
        }
        Err(error) => print!("{:#?}", error),
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    Fmt(std::fmt::Error),
    Int(std::num::ParseIntError),
}

macro_rules! parse {
    ($token:ident) => {{
        let token: i32 = $token.parse()?;
        token
    }};
    ($token:ident with operator) => {{
        let token: i32 = $token[..$token.len() - 1].parse()?;
        token
    }};
}

pub fn solve(input: &str) -> Result<String, Error> {
    use std::fmt::Write;

    let mut output = String::new();
    for line in input.lines() {
        let mut num = 0;
        let mut tokens = line.split_inclusive(|c| matches!(c, '+' | '-'));
        while let Some(token) = tokens.next() {
            match token.chars().last() {
                Some('+') => {
                    num += parse!(token with operator);
                }
                Some('-') => {
                    num += parse!(token with operator);

                    while let Some(token) = tokens.next() {
                        match token.chars().last() {
                            Some('+' | '-') => {
                                num -= parse!(token with operator);
                            }
                            _ => {
                                num -= parse!(token);
                            }
                        }
                    }
                }
                _ => {
                    num += parse!(token);
                }
            }
        }
        writeln!(output, "{}", num)?;
    }
    Ok(output)
}

impl From<std::fmt::Error> for Error {
    fn from(error: std::fmt::Error) -> Self {
        Error::Fmt(error)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(error: std::num::ParseIntError) -> Self {
        Error::Int(error)
    }
}
