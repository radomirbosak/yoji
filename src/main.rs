use std::fs::read_to_string;
use yoji::Yojijukugo;

fn read_db(filename: &str) -> Vec<Yojijukugo> {
    let mut result: Vec<Yojijukugo> = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.parse().unwrap())
    }
    result
}

fn main() {
    let filename = "path";
    println!("Reading database from {filename}");
    let db = read_db(filename);
    println!("Found {} entries:", db.len());
    for yoji in db {
        println!("{yoji}");
    }
}
