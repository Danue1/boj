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
    Float(std::num::ParseFloatError),
}

pub fn solve(input: &str) -> Result<String, Error> {
    use std::fmt::Write;

    let mut output = String::new();
    for line in input.lines() {
        let mut tokens = line.split_whitespace();
        match (tokens.next(), tokens.next()) {
            (Some(a), Some(b)) => {
                let a: f64 = a.parse()?;
                let b: f64 = b.parse()?;
                writeln!(&mut output, "{}", a / b)?;
            }
            _ => {
                //
            }
        }
    }
    Ok(output)
}

impl From<std::fmt::Error> for Error {
    fn from(error: std::fmt::Error) -> Self {
        Error::Fmt(error)
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(error: std::num::ParseFloatError) -> Self {
        Error::Float(error)
    }
}
