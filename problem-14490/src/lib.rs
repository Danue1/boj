#[derive(Debug, PartialEq)]
pub enum Error {
    Fmt(std::fmt::Error),
    Int(std::num::ParseIntError),
}

pub fn solve(input: &str) -> Result<String, Error> {
    use std::fmt::Write;

    let mut output = String::new();
    for line in input.lines() {
        let mut values = line.split(':');
        let m: u32 = values.next().unwrap().parse()?;
        let n: u32 = values.next().unwrap().parse()?;
        let gcd = gcd(m, n);
        writeln!(output, "{}:{}", m / gcd, n / gcd)?;
    }
    Ok(output)
}

fn gcd(m: u32, n: u32) -> u32 {
    match m {
        0 => n,
        _ => gcd(n % m, m),
    }
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
