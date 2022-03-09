#[derive(Debug, PartialEq)]
pub enum Error {
    Fmt(std::fmt::Error),
    Int(std::num::ParseIntError),
}

pub fn solve(input: &str) -> Result<String, Error> {
    use std::cmp::Ordering;
    use std::fmt::Write;

    let mut output = String::new();
    let mut lines = input.lines();
    let start_time: Time = lines.next().unwrap().parse()?;
    let end_time: Time = lines.next().unwrap().parse()?;
    let time_gap = match start_time.cmp(&end_time) {
        Ordering::Greater => {
            let end_time = Time {
                hour: end_time.hour + 24,
                ..end_time
            };

            end_time - start_time
        }
        Ordering::Equal => Time::FULL_TIME,
        Ordering::Less => end_time - start_time,
    };

    writeln!(output, "{}", time_gap)?;

    Ok(output)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Time {
    hour: i32,
    minute: i32,
    second: i32,
}

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}:{:02}", self.hour, self.minute, self.second)
    }
}

impl std::str::FromStr for Time {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values = s.split(':');

        Ok(Time {
            hour: values.next().unwrap().parse()?,
            minute: values.next().unwrap().parse()?,
            second: values.next().unwrap().parse()?,
        })
    }
}

impl std::ops::Sub for Time {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let lhs = self.timestamp() as i32;
        let rhs = rhs.timestamp() as i32;
        let diff = lhs - rhs;

        Time {
            hour: diff / HOUR,
            minute: (diff % HOUR / MINUTE).abs(),
            second: (diff % MINUTE).abs(),
        }
    }
}

const SECOND: i32 = 1;
const MINUTE: i32 = 60 * SECOND;
const HOUR: i32 = 60 * MINUTE;

impl Time {
    const FULL_TIME: Time = Time {
        hour: 24,
        minute: 0,
        second: 0,
    };

    const fn timestamp(&self) -> i32 {
        self.hour * HOUR + self.minute * MINUTE + self.second
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
