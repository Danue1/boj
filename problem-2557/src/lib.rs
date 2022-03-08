#[derive(Debug, PartialEq)]
pub enum Error {
    //
}

const SOURCE: &'static str = "Hello World!\n";

pub fn solve(_input: &str) -> Result<String, Error> {
    Ok(SOURCE.to_string())
}
