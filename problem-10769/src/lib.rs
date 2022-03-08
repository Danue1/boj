#[derive(Debug, PartialEq)]
pub enum Error {
    Fmt(std::fmt::Error),
}

pub fn solve(input: &str) -> Result<String, Error> {
    use std::fmt::Write;

    let mut output = String::new();
    for line in input.lines() {
        let happy = line.matches(":-)").count();
        let sad = line.matches(":-(").count();
        let feeling = match (happy, sad) {
            (happy, sad) if happy > sad => "happy",
            (happy, sad) if happy < sad => "sad",
            (0, 0) => "none",
            _ => "unsure",
        };
        writeln!(output, "{}", feeling)?;
    }
    Ok(output)
}

impl From<std::fmt::Error> for Error {
    fn from(error: std::fmt::Error) -> Self {
        Error::Fmt(error)
    }
}
