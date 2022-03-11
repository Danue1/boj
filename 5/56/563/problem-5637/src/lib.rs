#[derive(Debug, PartialEq)]
pub enum Error {
    Fmt(std::fmt::Error),
}

pub fn solve(input: &str) -> Result<String, Error> {
    use std::fmt::Write;

    let mut output = String::new();
    let mut longest_word = "";
    for word in input.split(|c| !matches!(c, 'a'..='z' | 'A'..='Z' | '-')) {
        if word.len() > longest_word.len() {
            longest_word = word;
        }
    }
    writeln!(output, "{}", longest_word.to_lowercase())?;

    Ok(output)
}

impl From<std::fmt::Error> for Error {
    fn from(error: std::fmt::Error) -> Self {
        Error::Fmt(error)
    }
}
