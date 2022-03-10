#[derive(Debug, PartialEq)]
pub enum Error {
    Fmt(std::fmt::Error),
    Int(std::num::ParseIntError),
}

pub fn solve(input: &str) -> Result<String, Error> {
    use std::fmt::Write;

    let mut output = String::new();
    let mut lines = input.lines();
    let max: usize = lines.next().unwrap().parse()?;
    'line: for line in lines.take(max) {
        let mut counts = [0u32; 26];
        let mut bytes = line.bytes();

        while let Some(current) = bytes.next() {
            let count = &mut counts[(current - b'A') as usize];
            match count {
                2 => match bytes.next() {
                    Some(next) if next == current => {
                        *count = 0;
                    }
                    _ => {
                        writeln!(output, "FAKE")?;
                        continue 'line;
                    }
                },
                _ => *count += 1,
            }
        }

        writeln!(output, "OK")?;
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
