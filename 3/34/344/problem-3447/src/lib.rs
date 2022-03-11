#[derive(Debug, PartialEq)]
pub enum Error {
    Fmt(std::fmt::Error),
}

pub fn solve(input: &str) -> Result<String, Error> {
    use std::fmt::Write;

    let mut output = String::new();
    let mut stack = vec![];

    macro_rules! clear {
        ($($tt:tt)+) => {{
            for step in &stack {
                write!(output, "{}", step)?;
            }
            stack.clear();
            write!(output, $($tt)+)?;
        }};
    }

    for line in input.lines() {
        for byte in line.bytes() {
            match byte {
                b'B' => stack.push(Step::B),
                b'U' => match stack.pop() {
                    Some(Step::B) => stack.push(Step::U),
                    Some(_) | None => clear!("U"),
                },
                b'G' => {
                    match stack.pop() {
                        Some(Step::U) => {
                            //
                        }
                        Some(_) => clear!("G"),
                        None => write!(output, "G")?,
                    }
                }
                _ => clear!("{}", byte as char),
            }
        }
        clear!("\n");
    }

    Ok(output)
}

enum Step {
    B,
    U,
}

impl std::fmt::Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Step::B => write!(f, "B"),
            Step::U => write!(f, "BU"),
        }
    }
}

impl From<std::fmt::Error> for Error {
    fn from(error: std::fmt::Error) -> Self {
        Error::Fmt(error)
    }
}
