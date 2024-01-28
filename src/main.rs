use std::fs::read_to_string;
use std::fmt;

#[derive(Default,PartialEq)]
struct Part {
    kanji: char,
    reading: String,
}

impl Part {
    fn new(kanji: char, reading: String) -> Part {
        Part { kanji, reading }
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

#[derive(Default,Debug,PartialEq)]
struct Yojijukugo(Part, Part, Part, Part);

impl fmt::Display for Yojijukugo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}{}", self.0, self.1, self.2, self.3)
    }
}


fn line_to_yoji(line: String) -> Yojijukugo {
    let mut split = line.split(" ");
    let text: String = split.next().unwrap().to_owned();
    let mut result: Vec<Part> = vec![];
    if let (Some(one), Some(two), Some(three), Some(four)) = (split.next(), split.next(), split.next(), split.next()) {
        for (kanji, reading) in text.chars().zip([one, two, three, four]) {
            println!("sdsdd");
            println!("{kanji}: {reading}");
            let p = Part::new(kanji, reading.to_owned());
            result.push(p);
        }
    } else {
        panic!("as");
    }
    // Yojijukugo(result.pop().unwrap(), result.pop().unwrap(), result.pop().unwrap(), result.pop().unwrap())

    // match 
    Yojijukugo(result.remove(0), result.remove(0), result.remove(0), result.remove(0))

    // match 



    // Default::default()
}

fn main() {
    line_to_yoji("自業自得 じ ごう じ とく".to_string());
    // let content = read_to_string("path").expect("Could not read file.");
    // println!("Content: {}", content);
    // let line_iter = content.lines();
    // for line in line_iter {
    //     println!("{}", line);
    //     let mut parts_iter = line.split(' ');
    //     let kanji = parts_iter.next().unwrap();
    //     let readings: Vec<&str> = parts_iter.collect();
    //     println!("Kanji: {}", kanji);
    //     println!("Readings: {:?}", readings);

    //     for (a,b) in kanji.chars().zip(readings) {
    //         println!("{}: {}", a, b);
    //     }
    // }
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
        assert_eq!(
            line_to_yoji("自業自得 じ ごう じ".to_string()),
            expected
        );
    }

    #[test]
    fn can_represent() {
        assert_eq!(jigo().to_string(), "自(じ)業(ごう)自(じ)得(とく)");
    }
}
