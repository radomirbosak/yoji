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

impl Yojijukugo {
    pub fn kanji(&self) -> String {
        format!(
            "{}{}{}{}",
            &self.0.kanji, &self.1.kanji, &self.2.kanji, &self.3.kanji
        )
    }
    pub fn kana(&self) -> String {
        format!(
            "{}{}{}{}",
            &self.0.reading, &self.1.reading, &self.2.reading, &self.3.reading
        )
    }
}

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

        // get iterators for kanji chars, and their readings
        let kanji_chars = split.next().ok_or(ParseYojijukugoError)?.chars();
        let readings = split;

        // zip these iterators and map into a Part struct
        let result = kanji_chars
            .zip(readings)
            .map(|(kanji, reading)| Part::new(kanji, reading));

        // collect the kanji parts into a 4-tuple
        let (a, b, c, d) = result
            .into_iter()
            .collect_tuple()
            .ok_or(ParseYojijukugoError)?;

        // construct yojijukugo from tuple
        Ok(Yojijukugo(a, b, c, d))
    }
}

fn main() {
    let jigo: Yojijukugo = "自業自得 じ ごう じ とく".parse().unwrap();
    println!("to_string: {}", jigo.to_string());
    println!("kanji: {}", jigo.kanji());
    println!("kana: {}", jigo.kana());
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

    #[test]
    fn kanji_method() {
        assert_eq!(jigo().kanji(), "自業自得");
    }
    #[test]
    fn kana_method() {
        assert_eq!(jigo().kana(), "じごうじとく");
    }
}
