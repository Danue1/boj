#[derive(Debug, PartialEq)]
pub enum Error {
    Fmt(std::fmt::Error),
    Int(std::num::ParseIntError),
}

pub fn solve(input: &str) -> Result<String, Error> {
    use std::fmt::Write;

    let (len, line) = {
        let mut lines = input.lines();
        let len: usize = lines.next().unwrap().parse()?;
        let line = lines.next().unwrap();
        (len, line)
    };
    let mut sum: u64 = 0;
    let mut num = 0;

    macro_rules! match_bytes {
        ($($byte:expr => $num:expr,)+) => {{
            let mut size = 0;

            for byte in line.as_bytes().iter().take(len) {
                match byte {
                    $(
                        $byte => {
                            size += 1;
                            num = num * 10 + $num
                        }
                    )+
                    _ => {
                        if size <= 6 {
                            sum += num;
                        }
                        size = 0;
                        num = 0;
                    }
                };
            }

            if size <= 6 {
                sum += num;
            }
        }};
    }

    match_bytes! {
        b'0' => 0,
        b'1' => 1,
        b'2' => 2,
        b'3' => 3,
        b'4' => 4,
        b'5' => 5,
        b'6' => 6,
        b'7' => 7,
        b'8' => 8,
        b'9' => 9,
    };

    let mut output = String::new();

    writeln!(output, "{}", sum)?;

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
