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
}

const CARD_BYTES: usize = 3;
const CARD_KIND_COUNT: usize = 4;
const CARD_SUIT_COUNT: usize = 13;
const CARD_COUNT: usize = CARD_KIND_COUNT * CARD_SUIT_COUNT;

pub fn solve(input: &str) -> Result<String, Error> {
    use std::fmt::Write;

    let mut output = String::new();
    'line: for line in input.lines() {
        macro_rules! error {
            () => {
                writeln!(output, "GRESKA")?;
                continue 'line;
            };
        }

        if line.len() > CARD_BYTES * CARD_COUNT {
            error!();
        }

        let mut spade_count = CARD_SUIT_COUNT as u8;
        let mut heart_count = CARD_SUIT_COUNT as u8;
        let mut diamond_count = CARD_SUIT_COUNT as u8;
        let mut club_count = CARD_SUIT_COUNT as u8;

        let mut spade_index = [false; CARD_SUIT_COUNT];
        let mut heart_index = [false; CARD_SUIT_COUNT];
        let mut diamond_index = [false; CARD_SUIT_COUNT];
        let mut club_index = [false; CARD_SUIT_COUNT];

        let mut chunks = line.as_bytes().chunks(CARD_BYTES);
        while let Some([kind, index @ ..]) = chunks.next() {
            macro_rules! match_index {
                (
                    match kind {
                        $(
                            $kind:pat => $card_index:expr,
                        )+
                    }

                    match index {
                        $(
                            $pattern:pat => $index:expr,
                        )+
                    }
                ) => {
                    let (count, card_index) = match kind {
                        $(
                            $kind => $card_index,
                        )+
                        _ => std::unreachable!(),
                    };

                    match index {
                        $(
                            $pattern => {
                                if card_index[$index] {
                                    error!();
                                }

                                card_index[$index] = true;
                                *count -= 1;
                            }
                        )+
                        _ => {
                            //
                        },
                    }
                };
            }

            match_index! {
                match kind {
                    b'P' => (&mut spade_count, &mut spade_index),
                    b'K' => (&mut heart_count, &mut heart_index),
                    b'H' => (&mut diamond_count, &mut diamond_index),
                    b'T' => (&mut club_count, &mut club_index),
                }

                match index {
                    [b'0', b'1'] => 0,
                    [b'0', b'2'] => 1,
                    [b'0', b'3'] => 2,
                    [b'0', b'4'] => 3,
                    [b'0', b'5'] => 4,
                    [b'0', b'6'] => 5,
                    [b'0', b'7'] => 6,
                    [b'0', b'8'] => 7,
                    [b'0', b'9'] => 8,
                    [b'1', b'0'] => 9,
                    [b'1', b'1'] => 10,
                    [b'1', b'2'] => 11,
                    [b'1', b'3'] => 12,
                }
            };
        }

        writeln!(
            output,
            "{} {} {} {}",
            spade_count, heart_count, diamond_count, club_count
        )?;
    }

    Ok(output)
}

impl From<std::fmt::Error> for Error {
    fn from(error: std::fmt::Error) -> Self {
        Error::Fmt(error)
    }
}
