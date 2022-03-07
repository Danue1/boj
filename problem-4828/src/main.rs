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
    InvalidChar(char),
    InvalidEscape(char),
    InvalidHex(char),
    UnexpectedToken(Token),
    InvalidTagName,
    NotEqualTagName(String, String),
    LoopDetected,
}

pub fn solve(input: &str) -> Result<String, Error> {
    use std::fmt::Write;

    let mut output = String::new();
    let mut solution = Solution::new(input);
    if let Some(answer) = solution.next() {
        write!(output, "{:#?}", answer)?;
        for answer in solution {
            write!(output, "\n{:#?}", answer)?;
        }
    }
    Ok(output)
}

impl From<std::fmt::Error> for Error {
    fn from(error: std::fmt::Error) -> Self {
        Error::Fmt(error)
    }
}

#[derive(Debug)]
pub struct Solution<'solution> {
    pub input: std::str::Lines<'solution>,
}

#[derive(PartialEq)]
pub enum Answer {
    Valid,
    Invalid,
}

impl<'solution> Solution<'solution> {
    #[inline]
    fn new(input: &'solution str) -> Self {
        Self {
            input: input.lines(),
        }
    }
}

impl<'solution> Iterator for Solution<'solution> {
    type Item = Answer;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.input.next() {
            Some(line) => Some(line.parse().unwrap()),
            None => None,
        }
    }
}

impl std::fmt::Debug for Answer {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Answer::Valid => write!(f, "valid"),
            Answer::Invalid => write!(f, "invalid"),
        }
    }
}

impl std::str::FromStr for Answer {
    type Err = Error;

    #[inline]
    fn from_str(source: &str) -> Result<Self, Self::Err> {
        let mut chars = TokenStream::new(source);
        match Xml::parse(&mut chars) {
            Ok(_) => Ok(Answer::Valid),
            Err(_) => Ok(Answer::Invalid),
        }
    }
}

pub struct TokenStream<'parse> {
    source: &'parse [u8],
    position: usize,
    current: Option<Token>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Eof,
    // <
    LeftChevron,
    // </
    LeftChevronSlash,
    // >
    RightChevron,
    // />
    SlashRightChevron,
    // 32..=127
    Char(char),
    Hex(String),
}

impl<'parse> TokenStream<'parse> {
    #[inline]
    pub fn new(source: &'parse str) -> Self {
        TokenStream {
            source: source.as_bytes(),
            position: 0,
            current: None,
        }
    }

    fn is_end(&self) -> bool {
        self.position >= self.source.len()
    }

    pub fn bump(&mut self) -> Result<Token, Error> {
        if self.is_end() {
            self.current = Some(Token::Eof);

            return Ok(Token::Eof);
        }

        let mut current = None;

        match self.source[self.position] {
            b'&' => {
                if let Some(index) = self.source[self.position + 1..]
                    .iter()
                    .position(|&b| b == b';')
                {
                    match &self.source[self.position + 1..self.position + 1 + index] {
                        [b'a', b'm', b'p'] => {
                            self.position += 1 + index + 1;
                            current = Some(Token::Char('&'));
                        }
                        [b'l', b't'] => {
                            self.position += 1 + index + 1;
                            current = Some(Token::Char('<'));
                        }
                        [b'g', b't'] => {
                            self.position += 1 + index + 1;
                            current = Some(Token::Char('>'));
                        }
                        [b'x', hex @ ..] if index != 1 && (index - 1) % 2 == 0 => {
                            const START: usize = 1;
                            let mut s = String::new();
                            for c in hex {
                                match c {
                                    b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F' => {
                                        s.push(c.clone() as char)
                                    }
                                    _ => return Err(Error::InvalidHex(c.clone() as char)),
                                }
                            }
                            self.position += 1 + START + index;
                            current = Some(Token::Hex(s));
                        }
                        _ => {
                            //
                        }
                    }
                }
            }
            b'<' => {
                self.position += 1;
                if !self.is_end() {
                    match self.source[self.position] {
                        b'/' => {
                            self.position += 1;
                            current = Some(Token::LeftChevronSlash);
                        }
                        _ => current = Some(Token::LeftChevron),
                    }
                } else {
                    current = Some(Token::LeftChevron)
                }
            }
            b'>' => {
                self.position += 1;
                current = Some(Token::RightChevron);
            }
            b'/' => {
                self.position += 1;
                if !self.is_end() {
                    match self.source[self.position] {
                        b'>' => {
                            self.position += 1;
                            current = Some(Token::SlashRightChevron);
                        }
                        _ => {
                            current = Some(Token::Char('/'));
                        }
                    }
                } else {
                    current = Some(Token::Char('/'));
                }
            }
            c @ b' '..=127 => {
                self.position += 1;
                current = Some(Token::Char(c as char));
            }
            _ => {}
        }

        self.current = current.clone();

        if let Some(current) = current {
            Ok(current)
        } else {
            Err(Error::InvalidChar(self.source[self.position] as char))
        }
    }

    fn peek(&mut self) -> Result<Token, Error> {
        if let Some(ref current) = self.current {
            Ok(current.clone())
        } else {
            self.bump()
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Xml {
    pub children: Vec<Element>,
}

#[derive(Debug, PartialEq)]
pub enum Element {
    Tag(Tag),
    Text(Text),
}

#[derive(Debug, PartialEq)]
pub struct Tag {
    pub name: Name,
    pub children: Vec<Element>,
}

#[derive(Debug, PartialEq)]
pub struct Name {
    pub raw: String,
}

#[derive(Debug, PartialEq)]
pub struct Text {
    pub raw: String,
}

impl Xml {
    pub fn parse(s: &mut TokenStream) -> Result<Self, Error> {
        let mut children = vec![];
        while !s.is_end() {
            let previous_position = s.position;
            children.push(Element::parse(s)?);
            if s.position == previous_position {
                return Err(Error::LoopDetected);
            }
        }
        Ok(Xml { children })
    }
}

impl Element {
    fn parse(s: &mut TokenStream) -> Result<Self, Error> {
        match s.peek()? {
            Token::LeftChevron => {
                s.bump()?;

                let name = Name::parse(s)?;

                match s.peek()? {
                    // <ident/>
                    Token::SlashRightChevron => {
                        s.bump()?;

                        Ok(Element::Tag(Tag {
                            name,
                            children: vec![],
                        }))
                    }
                    // <ident>
                    Token::RightChevron => {
                        s.bump()?;

                        let mut children = vec![];
                        while s.peek()? != Token::LeftChevronSlash {
                            let previous_position = s.position;

                            children.push(Element::parse(s)?);

                            if s.position == previous_position {
                                if !matches!(
                                    s.peek()?,
                                    Token::LeftChevron | Token::LeftChevronSlash
                                ) {
                                    break;
                                }
                                return Err(Error::LoopDetected);
                            }
                        }

                        s.bump()?; // </

                        let ident = Name::parse(s)?;
                        if name != ident {
                            return Err(Error::NotEqualTagName(name.raw, ident.raw));
                        }

                        match s.peek()? {
                            Token::RightChevron => {
                                s.bump()?;

                                Ok(Element::Tag(Tag { name, children }))
                            }
                            token => Err(Error::UnexpectedToken(token)),
                        }
                    }
                    token => Err(Error::UnexpectedToken(token)),
                }
            }
            Token::LeftChevronSlash | Token::SlashRightChevron => Ok(Element::Text(Text {
                raw: "".to_string(),
            })),
            _ => Ok(Element::Text(Text::parse(s)?)),
        }
    }
}

impl Name {
    fn parse(s: &mut TokenStream) -> Result<Self, Error> {
        let mut raw = String::new();
        loop {
            match s.peek()? {
                Token::Char(c @ ('0'..='9' | 'a'..='z')) => {
                    s.bump()?;
                    raw.push(c);
                }
                _ => break,
            }
        }

        if raw.is_empty() {
            Err(Error::InvalidTagName)
        } else {
            Ok(Name { raw })
        }
    }
}

impl Text {
    fn parse(s: &mut TokenStream) -> Result<Self, Error> {
        let mut raw = String::new();
        loop {
            match s.peek()? {
                Token::Char(c) => {
                    s.bump()?;
                    raw.push(c)
                }
                Token::Hex(ref c) => {
                    s.bump()?;
                    raw.push_str(c);
                }
                _ => break,
            }
        }
        Ok(Text { raw })
    }
}
