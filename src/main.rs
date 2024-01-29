use yoji::Yojijukugo;

fn main() {
    let jigo: Yojijukugo = "自業自得 じ ごう じ とく".parse().unwrap();
    println!("to_string: {}", jigo.to_string());
    println!("kanji: {}", jigo.kanji());
    println!("kana: {}", jigo.kana());
    println!("kanji_kana: {}", jigo.kanji_kana());
    println!("kana_kanji: {}", jigo.kana_kanji());
}
