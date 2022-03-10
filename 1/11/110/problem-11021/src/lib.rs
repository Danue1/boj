#[derive(Debug, PartialEq)]
pub enum Error {
    Fmt(std::fmt::Error),
    Int(std::num::ParseIntError),
}

pub fn solve(input: &str) -> Result<String, Error> {
    use std::fmt::Write;

    let mut output = String::new();
    let mut lines = input.lines().enumerate();
    lines.next();
    for (index, line) in lines {
        let mut tokens = line.split_ascii_whitespace();
        let a: i32 = tokens.next().unwrap().parse()?;
        let b: i32 = tokens.next().unwrap().parse()?;
        writeln!(output, "Case #{}: {}", index, a + b)?;
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
