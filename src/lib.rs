use std::fmt;

#[derive(Default, PartialEq)]
pub struct Part {
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
pub struct Yojijukugo([Part; 4]);

impl Yojijukugo {
    pub fn kanji(&self) -> String {
        format!(
            "{}{}{}{}",
            &self.0[0].kanji, &self.0[1].kanji, &self.0[2].kanji, &self.0[3].kanji
        )
    }
    pub fn kana(&self) -> String {
        format!(
            "{}{}{}{}",
            &self.0[0].reading, &self.0[1].reading, &self.0[2].reading, &self.0[3].reading
        )
    }
    pub fn kanji_kana(&self) -> String {
        format!(
            "{}{}{}{}",
            &self.0[0].kanji, &self.0[1].kanji, &self.0[2].reading, &self.0[3].reading
        )
    }
    pub fn kana_kanji(&self) -> String {
        format!(
            "{}{}{}{}",
            &self.0[0].reading, &self.0[1].reading, &self.0[2].kanji, &self.0[3].kanji
        )
    }
}

impl fmt::Display for Yojijukugo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}{}", self.0[0], self.0[1], self.0[2], self.0[3])
    }
}

#[derive(Debug, PartialEq)]
pub struct ParseYojijukugoError;

impl std::str::FromStr for Yojijukugo {
    type Err = ParseYojijukugoError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');

        // get iterators for kanji chars, and their readings
        let kanji_chars = split.next().ok_or(ParseYojijukugoError)?.chars();
        let readings = split;

        // zip these iterators and map into a Part struct
        let parts: Vec<Part> = kanji_chars
            .zip(readings)
            .map(|(kanji, reading)| Part::new(kanji, reading))
            .collect();

        Ok(Yojijukugo(parts.try_into().or(Err(ParseYojijukugoError))?))
    }
}

pub fn get_sample1234() -> Yojijukugo {
    Yojijukugo([
        Part {
            kanji: '一',
            reading: String::from("いち"),
        },
        Part {
            kanji: '二',
            reading: String::from("に"),
        },
        Part {
            kanji: '三',
            reading: String::from("さん"),
        },
        Part {
            kanji: '四',
            reading: String::from("し"),
        },
    ])
}
