use rand::Rng;
use std::io::Write;
use rand::prelude::IteratorRandom;
use std::fs::read_to_string;
use yoji::Yojijukugo;
// use rand::seq::SliceRandom;
use std::io;

fn read_db(filename: &str) -> Vec<Yojijukugo> {
    let mut result: Vec<Yojijukugo> = Vec::new();

    for (i, line) in read_to_string(filename).unwrap().lines().enumerate() {
        match line.parse() {
            Ok(record) => {result.push(record);}
            Err(_) => {panic!("Could not parse line {}: {:?}", i + 1, line);}
        }
    }
    result
}

fn choose<T>(raw: &mut Vec<T>) -> Option<T> {
    let i = (0..raw.len()).choose(&mut rand::thread_rng())?;
    Some(raw.swap_remove(i))
}

fn main() {
    let filename = "kanken3_yojijukugo.txt";
    println!("Reading database from {filename}");
    let mut db: Vec<Yojijukugo> = read_db(filename);
    println!("Found {} entries:", db.len());
    println!("Starting test session.");
    let mut guess = String::new();
    let mut rng = rand::thread_rng();
    let dbsize: usize = db.len();
    let mut test_idx: usize = 1;
    while let Some(random_yoji) = choose(&mut db) {
        println!("Test {}/{}", test_idx, dbsize);

        // choose test at random
        let zadanie = match rng.gen::<bool>() {
            true => random_yoji.kanji_kana(),
            false => random_yoji.kana_kanji(),
        };
        print!("{zadanie}");
        io::stdout().flush().unwrap();

        // wait for enter key
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        // print the correct solution
        println!("{}\n", random_yoji.kanji());
        test_idx += 1;
    }
}
