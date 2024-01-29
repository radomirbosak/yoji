#[cfg(test)]
mod tests {
    use yoji::{Part, Yojijukugo};

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
