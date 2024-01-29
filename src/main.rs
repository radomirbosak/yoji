use itertools::Itertools;
use std::fmt;
// use std::fs::read_to_string;

#[derive(Default, PartialEq)]
struct Part {
    kanji: char,
    reading: String,
}

impl Part {
    fn new(kanji: char, reading: impl Into<String>) -> Part {
        Part {
            kanji,
            reading: reading.into(),
        }
    }
}

impl fmt::Debug for Part {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.kanji, self.reading)
    }
}

impl fmt::Display for Part {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.kanji, self.reading)
    }
}

#[derive(Default, Debug, PartialEq)]
struct Yojijukugo(Part, Part, Part, Part);

impl fmt::Display for Yojijukugo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}{}", self.0, self.1, self.2, self.3)
    }
}

#[derive(Debug, PartialEq)]
struct ParseYojijukugoError;

impl std::str::FromStr for Yojijukugo {
    type Err = ParseYojijukugoError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');
        let result = split
            .next()
            .ok_or(ParseYojijukugoError)?
            .chars()
            .zip(split)
            .map(|(kanji, reading)| Part::new(kanji, reading));

        let (a, b, c, d) = result
            .into_iter()
            .collect_tuple()
            .ok_or(ParseYojijukugoError)?;
        Ok(Yojijukugo(a, b, c, d))
    }
}

fn main() {
    let a: Yojijukugo = "自業自得 じ ごう じ とく".parse().unwrap();
    println!("{a}");
}

#[cfg(test)]
mod tests {
    use crate::{Part, Yojijukugo};

    fn jigo() -> Yojijukugo {
        Yojijukugo(
            Part {
                kanji: '自',
                reading: String::from("じ"),
            },
            Part {
                kanji: '業',
                reading: String::from("ごう"),
            },
            Part {
                kanji: '自',
                reading: String::from("じ"),
            },
            Part {
                kanji: '得',
                reading: String::from("とく"),
            },
        )
    }

    #[test]
    fn can_parse() {
        let expected: Yojijukugo = jigo();
        assert_eq!("自業自得 じ ごう じ とく".parse(), Ok(expected));
    }

    #[test]
    #[should_panic]
    fn cannot_parse() {
        let expected: Yojijukugo = jigo();
        assert_eq!("自業自得 じ ごう じ".parse(), Ok(expected));
    }

    #[test]
    fn can_represent() {
        assert_eq!(jigo().to_string(), "自(じ)業(ごう)自(じ)得(とく)");
    }
}
