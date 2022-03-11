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
    use std::fmt::Write;

    let mut output = String::new();
    let (count_line, word_line) = input.split_once('\n').unwrap();

    let mut first: Option<String> = None;

    macro_rules! swap {
        (@ $($word1:ident)* $(& $word2:ident)*) => {{
            if $($word1)* $($word2)* .len() >= 2 {
                match first {
                    Some(f) if $($word1 < f)* $($word2 < &f)* => {
                        first = Some($($word1)* $($word2)* .to_string());
                    }
                    Some(_) => {
                        //
                    }
                    None => {
                        first = Some($($word1)* $($word2)* .to_string());
                    }
                }
            }
        }};
        (if $word:ident < &first) => {
            swap!(@ $word);
        };
        (if $word:ident < first) => {
            swap!(@ & $word);
        };
    }

    let lines: Vec<&str> = word_line.lines().collect();
    for word in lines.iter().flat_map(|line| line.split('#')) {
        swap!(if word < first);
    }

    let (row_count, column_count) = count_line.split_once(' ').unwrap();
    let row_count: usize = row_count.parse()?;
    let column_count: usize = column_count.parse()?;
    let mut word = String::new();
    for column in 0..column_count {
        word.clear();
        for row in 0..row_count {
            match lines[row].as_bytes()[column] {
                b'#' => {
                    swap!(if word < &first);
                    word.clear();
                }
                c @ b'a'..=b'z' => word.push(c as char),
                _ => {
                    //
                }
            }
        }

        if word.len() >= 2 {
            swap!(if word < &first);
        }
    }

    if let Some(first) = first {
        writeln!(output, "{}", first)?;
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
