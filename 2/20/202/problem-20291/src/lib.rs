#[derive(Debug, PartialEq)]
pub enum Error {
    Fmt(std::fmt::Error),
    Int(std::num::ParseIntError),
}

pub fn solve(input: &str) -> Result<String, Error> {
    use std::collections::btree_map::Entry;
    use std::collections::BTreeMap;
    use std::fmt::Write;

    let mut output = String::new();
    let mut extensions = BTreeMap::new();
    for extension in input.split(|c| matches!(c, '\n' | '.')).skip(2).step_by(2) {
        match extensions.entry(extension) {
            Entry::Vacant(entry) => {
                entry.insert(1u16);
            }
            Entry::Occupied(mut entry) => {
                *entry.get_mut() += 1;
            }
        }
    }

    for (extension, count) in extensions {
        writeln!(output, "{} {}", extension, count)?;
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
