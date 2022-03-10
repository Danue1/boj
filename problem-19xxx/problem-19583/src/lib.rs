#[derive(Debug, PartialEq)]
pub enum Error {
    Fmt(std::fmt::Error),
    Int(std::num::ParseIntError),
}

pub fn solve(input: &str) -> Result<String, Error> {
    use std::collections::HashMap;
    use std::fmt::Write;

    let mut output = String::new();
    let mut lines = input.lines().peekable();

    let grand: Grand = lines.next().unwrap().parse()?;
    let mut members = HashMap::new();

    while let Some(line) = lines.peek() {
        let (time, name) = line.split_once(' ').unwrap();
        let time: Time = time.parse()?;
        match time {
            _ if time <= grand.open => {
                lines.next();
                members.entry(name).or_insert(false);
            }
            _ => break,
        }
    }

    for line in lines {
        let (time, name) = line.split_once(' ').unwrap();
        let time: Time = time.parse()?;
        match time {
            _ if time > grand.close => break,
            _ if time >= grand.end => {
                members.entry(name).and_modify(|check| *check = true);
            }
            _ => {
                //
            }
        }
    }

    let count = members.iter().filter(|(_, &check)| check).count();
    writeln!(output, "{}", count)?;

    Ok(output)
}

struct Grand {
    open: Time,
    end: Time,
    close: Time,
}

impl std::str::FromStr for Grand {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut timeline = s.split_whitespace();

        Ok(Grand {
            open: timeline.next().unwrap().parse()?,
            end: timeline.next().unwrap().parse()?,
            close: timeline.next().unwrap().parse()?,
        })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Time {
    hour: u8,
    minute: u8,
}

impl std::str::FromStr for Time {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split(':');

        Ok(Time {
            hour: tokens.next().unwrap().parse()?,
            minute: tokens.next().unwrap().parse()?,
        })
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
