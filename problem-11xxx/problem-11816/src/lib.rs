#[derive(Debug, PartialEq)]
pub enum Error {
    Fmt(std::fmt::Error),
    Int(std::num::ParseIntError),
}

pub fn solve(input: &str) -> Result<String, Error> {
    use std::fmt::Write;

    let mut output = String::new();
    for line in input.lines() {
        match line.as_bytes()[0..2] {
            [b'0', b'x'] => {
                let num = u32::from_str_radix(&line[2..], 16)?;
                writeln!(output, "{}", num)?;
            }
            [b'0', _] => {
                let num = u32::from_str_radix(&line[1..], 8)?;
                writeln!(output, "{}", num)?;
            }
            _ => writeln!(output, "{}", line)?,
        };
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
