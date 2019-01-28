use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let n = read_n();
    let mut phone_book = HashMap::new();

    for _ in 0..n {
        let entry = read_string();
        let mut iterator = entry.trim().split_whitespace();
        let name = iterator.next().unwrap().to_owned();
        let phone = iterator.next().unwrap().to_owned();

        phone_book.insert(name, phone);
    }

    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    for query in lines {
        let q = query.unwrap();
        let name = q.trim();
        match phone_book.get::<str>(&name) {
            Some(x) => println!("{}={}", name, x),
            None => println!("Not found")
        }
    }
}

fn read_n() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn read_string() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}