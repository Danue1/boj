fn main() {
    use std::io::Read;

    let mut io = std::io::stdin();
    let mut input = String::new();
    match io.read_to_string(&mut input) {
        Ok(_) => match solve(&input) {
            Ok(output) => print!("{}", output),
            Err(error) => print!("{:#?}", error),
        },
        Err(error) => print!("{:#?}", error),
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    Fmt(std::fmt::Error),
    Int(std::num::ParseIntError),
}

pub fn solve(input: &str) -> Result<String, Error> {
    use std::fmt::Write;

    let mut output = String::new();
    let mut lines = input.lines();
    let max = lines.next().unwrap().parse()?;
    let mut nums: Vec<&str> = lines
        .take(max)
        .flat_map(|line| line.split(|c| !matches!(c, '0'..='9')))
        .filter_map(|num| {
            if !num.is_empty() {
                if let Some(index) = num.find(|c| matches!(c, '1'..='9')) {
                    Some(&num[index..])
                } else {
                    Some("0")
                }
            } else {
                None
            }
        })
        .collect();

    nums.sort_by(|a, b| match a.len().cmp(&b.len()) {
        std::cmp::Ordering::Equal => a.cmp(b),
        other => other,
    });

    for num in nums {
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
