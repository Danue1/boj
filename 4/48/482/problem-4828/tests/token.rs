use problem_4828::{Error, Token, TokenStream};

#[test]
fn left_chevron() {
    let source = "<";
    let mut stream = TokenStream::new(source);

    assert_eq!(stream.bump(), Ok(Token::LeftChevron));
}

#[test]
fn left_chevron_slash() {
    let source = "</";
    let mut stream = TokenStream::new(source);

    assert_eq!(stream.bump(), Ok(Token::LeftChevronSlash));
}

#[test]
fn right_chevron() {
    let source = ">";
    let mut stream = TokenStream::new(source);

    assert_eq!(stream.bump(), Ok(Token::RightChevron));
}

#[test]
fn right_chevron_slash() {
    let source = "/>";
    let mut stream = TokenStream::new(source);

    assert_eq!(stream.bump(), Ok(Token::SlashRightChevron));
}

#[test]
fn less_than() {
    let source = "&lt;";
    let mut stream = TokenStream::new(source);

    assert_eq!(stream.bump(), Ok(Token::Char('<')));
}

#[test]
fn greater_than() {
    let source = "&gt;";
    let mut stream = TokenStream::new(source);

    assert_eq!(stream.bump(), Ok(Token::Char('>')));
}

#[test]
fn ampersand() {
    let source = "&amp;";
    let mut stream = TokenStream::new(source);

    assert_eq!(stream.bump(), Ok(Token::Char('&')));
}

#[test]
fn hex0() {
    let source = "&x;";
    let mut stream = TokenStream::new(source);

    assert_eq!(stream.bump(), Err(Error::InvalidChar('&')));
}

#[test]
fn hex1() {
    let source = "&xa;";
    let mut stream = TokenStream::new(source);

    assert_eq!(stream.bump(), Err(Error::InvalidChar('&')));
}

#[test]
fn hex2() {
    let source = "&x12;";
    let mut stream = TokenStream::new(source);

    assert_eq!(stream.bump(), Ok(Token::Hex("12".to_string())));
}

#[test]
fn hex4() {
    let source = "&xABCD;";
    let mut stream = TokenStream::new(source);

    assert_eq!(stream.bump(), Ok(Token::Hex("ABCD".to_string())));
}
