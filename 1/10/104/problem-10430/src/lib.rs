#[derive(Debug, PartialEq)]
pub enum Error {
    Fmt(std::fmt::Error),
    Int(std::num::ParseIntError),
}

pub fn solve(input: &str) -> Result<String, Error> {
    use std::fmt::Write;

    let mut output = String::new();
    for line in input.lines() {
        let mut values = line.split_whitespace();
        let a: u32 = values.next().unwrap().parse()?;
        let b: u32 = values.next().unwrap().parse()?;
        let c: u32 = values.next().unwrap().parse()?;
        writeln!(output, "{}", (a + b) % c)?;
        writeln!(output, "{}", ((a % c) + (b % c)) % c)?;
        writeln!(output, "{}", (a * b) % c)?;
        writeln!(output, "{}", ((a % c) * (b % c)) % c)?;
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
