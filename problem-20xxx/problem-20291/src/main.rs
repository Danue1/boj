fn main() {
    use std::io::Read;

    let mut io = std::io::stdin();
    let mut input = String::new();
    match io.read_to_string(&mut input) {
        Ok(_) => match solve(&input) {
            Ok(output) => print!("{}", output),
            Err(error) => print!("{:#?}", error),
        }
        Err(error) => print!("{:#?}", error),
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    Fmt(std::fmt::Error),
    Int(std::num::ParseIntError),
}

pub fn solve(input: &str) -> Result<String, Error> {
    use std::collections::HashMap;
    use std::fmt::Write;

    let mut output = String::new();
    let mut lines = input.lines();
    let mut extensions = HashMap::new();
    let max: usize = lines.next().unwrap().parse()?;
    for extension in lines.take(max).filter_map(|file| file.split('.').last()) {
        extensions
            .entry(extension)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    let mut sorted_extensions: Vec<&str> = extensions.keys().map(|&key| key).collect();
    sorted_extensions.sort();
    for extension in sorted_extensions {
        writeln!(
            output,
            "{} {}",
            extension,
            extensions.get(extension).unwrap_or(&0)
        )?;
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
