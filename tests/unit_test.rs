#[cfg(test)]
mod tests {
    use yoji::{get_sample1234, Yojijukugo};

    #[test]
    fn can_parse() {
        let expected: Yojijukugo = get_sample1234();
        assert_eq!("一二三四 いち に さん し".parse(), Ok(expected));
    }

    #[test]
    #[should_panic]
    fn cannot_parse() {
        let expected: Yojijukugo = get_sample1234();
        assert_eq!("一二三四 いち に さん".parse(), Ok(expected));
    }

    #[test]
    fn can_represent() {
        assert_eq!(get_sample1234().to_string(), "一(いち)二(に)三(さん)四(し)");
    }

    #[test]
    fn kanji_method() {
        assert_eq!(get_sample1234().kanji(), "一二三四");
    }
    #[test]
    fn kana_method() {
        assert_eq!(get_sample1234().kana(), "いちにさんし");
    }
}
