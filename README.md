# Yoji

yojijukugo test Rust library


Usage:

```rust
fn main() {
    let jigo: Yojijukugo = "自業自得 じ ごう じ とく".parse().unwrap();
    println!("to_string: {}", jigo.to_string());
    println!("kanji: {}", jigo.kanji());
    println!("kana: {}", jigo.kana());
}
```

output:

```
to_string: 自(じ)業(ごう)自(じ)得(とく)
kanji: 自業自得
kana: じごうじとく
```