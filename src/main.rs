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
        Part { kanji, reading:reading.into() }
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

fn line_to_yoji(line: String) -> Yojijukugo {
    let mut split = line.split(' ');
    // let text: &str = split.next().unwrap();
    let result = split.next().unwrap()
        .chars()
        .zip(split)
        .map(|(kanji, reading)| Part::new(kanji, reading));

    let (a, b, c, d) = result.into_iter().collect_tuple().unwrap();
    Yojijukugo(a, b, c, d)
}

fn main() {
    let a = line_to_yoji("自業自得 じ ごう じ とく".to_string());
    println!("{a}");
}

#[cfg(test)]
mod tests {
    use crate::line_to_yoji;
    use crate::{Part, Yojijukugo};

    fn jigo() -> Yojijukugo {
        Yojijukugo(
            Part::new('自', String::from("じ")),
            Part::new('業', String::from("ごう")),
            Part::new('自', String::from("じ")),
            Part::new('得', String::from("とく")),
        )
    }

    #[test]
    fn can_parse() {
        let expected: Yojijukugo = jigo();
        assert_eq!(
            line_to_yoji("自業自得 じ ごう じ とく".to_string()),
            expected
        );
    }

    #[test]
    #[should_panic]
    fn cannot_parse() {
        let expected: Yojijukugo = jigo();
        assert_eq!(line_to_yoji("自業自得 じ ごう じ".to_string()), expected);
    }

    #[test]
    fn can_represent() {
        assert_eq!(jigo().to_string(), "自(じ)業(ごう)自(じ)得(とく)");
    }
}
