#[derive(Debug, PartialEq)]
pub enum Error {
    //
}

const SOURCE: &'static str = r#"|\_/|
|q p|   /}
( 0 )"""\
|"^"`    |
||_/=\\__|
"#;

pub fn solve(_input: &str) -> Result<String, Error> {
    Ok(SOURCE.to_string())
}
