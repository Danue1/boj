#[derive(Debug, PartialEq)]
pub enum Error {
    Fmt(std::fmt::Error),
    Int(std::num::ParseFloatError),
}

pub fn solve(input: &str) -> Result<String, Error> {
    use std::fmt::Write;

    let mut output = String::new();
    for line in input.lines() {
        let mut token = line
            .split(|c| !matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9'))
            .filter(|s| !s.is_empty());
        let month = match token.next().unwrap() {
            "January" => 0,
            "February" => 1,
            "March" => 2,
            "April" => 3,
            "May" => 4,
            "June" => 5,
            "July" => 6,
            "August" => 7,
            "September" => 8,
            "October" => 9,
            "November" => 10,
            "December" => 11,
            _ => std::unreachable!(),
        };
        let date: f64 = token.next().unwrap().parse()?;
        let year: f64 = token.next().unwrap().parse()?;
        let hour: f64 = token.next().unwrap().parse()?;
        let minute: f64 = token.next().unwrap().parse()?;

        let is_leap_year = year % 400.0 == 0.0 || (year % 4.0 == 0.0 && year % 100.0 != 0.0);
        let days_in_month = [
            31.0,
            if is_leap_year { 29.0 } else { 28.0 },
            31.0,
            30.0,
            31.0,
            30.0,
            31.0,
            31.0,
            30.0,
            31.0,
            30.0,
            31.0,
        ];
        let mut days = date;
        for index in 0..month {
            days += days_in_month[index];
        }

        let all_time = days * DAY + hour * HOUR + minute - DAY;
        let all_day = if is_leap_year { 366.0 } else { 365.0 } * DAY;
        writeln!(output, "{}", all_time / all_day * 100.0)?;
    }

    Ok(output)
}

const MINUTE: f64 = 1.0;
const HOUR: f64 = MINUTE * 60.0;
const DAY: f64 = HOUR * 24.0;

impl From<std::fmt::Error> for Error {
    fn from(error: std::fmt::Error) -> Self {
        Error::Fmt(error)
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(error: std::num::ParseFloatError) -> Self {
        Error::Int(error)
    }
}
