#[derive(Debug, PartialEq)]
pub enum Error {
    Fmt(std::fmt::Error),
    Int(std::num::ParseIntError),
}

pub fn solve(input: &str) -> Result<String, Error> {
    use std::fmt::Write;

    let mut output = String::new();
    let mut lines = input.lines();
    let count = lines.next().unwrap().parse()?;
    for line in lines.take(count) {
        let kind = line.as_bytes()[0];
        let address = &line[2..];
        match kind {
            b'1' => {
                let address = address
                    .split('.')
                    .filter_map(|num| num.parse().ok())
                    .fold(0, |a: u64, b: u64| (a << 8) | b);
                writeln!(output, "{}", address)?;
            }
            b'2' => {
                let address: u64 = address.parse()?;
                writeln!(
                    output,
                    "{}.{}.{}.{}.{}.{}.{}.{}",
                    (address >> 56) as u8,
                    (address >> 48) as u8,
                    (address >> 40) as u8,
                    (address >> 32) as u8,
                    (address >> 24) as u8,
                    (address >> 16) as u8,
                    (address >> 8) as u8,
                    (address >> 0) as u8,
                )?;
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

impl From<std::num::ParseIntError> for Error {
    fn from(error: std::num::ParseIntError) -> Self {
        Error::Int(error)
    }
}
