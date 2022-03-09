#[derive(Debug, PartialEq)]
pub enum Error {
    Fmt(std::fmt::Error),
    Int(std::num::ParseIntError),
}

pub fn solve(input: &str) -> Result<String, Error> {
    use std::fmt::Write;

    let mut output = String::new();
    let mut lines = input.lines();

    let count: usize = lines.next().unwrap().parse()?;
    let mut lines = lines.take(count * 3);

    while let Some(instructions) = lines.next() {
        let item_count: usize = lines.next().unwrap().parse()?;

        let mut head = 0;
        let mut tail = 0;
        let mut direction = Direction::Head;

        for instruction in instructions.bytes() {
            match instruction {
                b'R' => {
                    direction = match direction {
                        Direction::Head => Direction::Tail,
                        Direction::Tail => Direction::Head,
                    };
                }
                b'D' => match direction {
                    Direction::Head => {
                        head += 1;
                    }
                    Direction::Tail => {
                        tail += 1;
                    }
                },
                _ => {
                    std::unreachable!();
                }
            }
        }

        let values = lines.next().unwrap();

        macro_rules! values {
            ($split:ident, $count:ident) => {{
                let values = &values[1..values.len() - 1];
                let mut values = values
                    .$split(',')
                    .skip($count)
                    .take(item_count - (head + tail));
                write!(output, "[")?;
                if let Some(value) = values.next() {
                    write!(output, "{}", value)?;
                    for value in values {
                        write!(output, ",{}", value)?;
                    }
                }
                writeln!(output, "]")?;
            }};
        }

        if tail <= item_count && head <= (item_count - tail) {
            match direction {
                Direction::Head => values!(split, head),
                Direction::Tail => values!(rsplit, tail),
            }
        } else {
            writeln!(output, "error")?;
        }
    }

    Ok(output)
}

#[derive(Debug, PartialEq)]
enum Direction {
    Head,
    Tail,
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
