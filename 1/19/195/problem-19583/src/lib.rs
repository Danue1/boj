#[derive(Debug, PartialEq)]
pub enum Error {
    Fmt(std::fmt::Error),
    Int(std::num::ParseIntError),
}

pub fn solve(input: &str) -> Result<String, Error> {
    use std::collections::HashMap;
    use std::fmt::Write;

    let mut output = String::new();
    let (grand, lines) = input.split_once('\n').unwrap();
    let grand: Grand = grand.parse()?;
    let mut lines = lines
        .lines()
        .map(|line| {
            let (time, name) = line.split_once(' ').unwrap();
            let time: Time = time.parse().unwrap();
            (time, name)
        })
        .peekable();
    let mut members = HashMap::new();

    while let Some(&(time, name)) = lines.peek() {
        if time > grand.open {
            break;
        }

        lines.next();
        members.insert(name, 0);
    }

    for (time, name) in lines {
        if time >= grand.close {
            if time > grand.end {
                break;
            }

            members.entry(name).and_modify(|check| *check = 1);
        }
    }

    let count: usize = members.values().sum();
    writeln!(output, "{}", count)?;

    Ok(output)
}

struct Grand {
    open: Time,
    close: Time,
    end: Time,
}

impl std::str::FromStr for Grand {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (open, timeline) = s.split_once(' ').unwrap();
        let (close, end) = timeline.split_once(' ').unwrap();

        Ok(Grand {
            open: open.parse()?,
            close: close.parse()?,
            end: end.parse()?,
        })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Time(u16);

impl std::str::FromStr for Time {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hour, minute) = s.split_once(':').unwrap();
        let hour: u16 = hour.parse()?;
        let minute: u16 = minute.parse()?;

        Ok(Time((hour << 8) | minute))
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
