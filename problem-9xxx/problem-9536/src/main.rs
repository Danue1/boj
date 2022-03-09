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

use std::collections::HashSet;

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
    for _ in 0..max {
        let sounds = lines.next().unwrap();
        let mut animals = HashSet::new();

        while let Some(animal) = lines.next() {
            match animal {
                "what does the fox say?" => break,
                _ => {
                    let sound = animal.split_whitespace().nth(2).unwrap();
                    animals.insert(sound);
                }
            }
        }

        let mut sounds = sounds
            .split_whitespace()
            .filter(|sound| !animals.contains(sound));
        if let Some(sound) = sounds.next() {
            write!(output, "{}", sound)?;
            for sound in sounds {
                write!(output, " {}", sound)?;
            }
            writeln!(output)?;
        }
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
