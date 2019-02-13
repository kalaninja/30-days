use std::collections::HashMap;
use std::io::BufRead;

fn main() {
    let n = read_ln().trim().parse().unwrap();
    let phone_book = (0..n).map(|_| read_entry())
        .fold(HashMap::new(), |mut acc, x| {
            acc.insert(x.0, x.1);
            acc
        });

    std::io::stdin().lock().lines()
        .map(|x| x.unwrap().trim().to_string())
        .for_each(|name| {
            match phone_book.get::<str>(&name) {
                Some(phone) => println!("{}={}", name, phone),
                None => println!("Not found")
            }
        });
}

fn read_ln() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

fn read_entry() -> (String, String) {
    let ln = read_ln();
    let mut iter = ln.trim().split_whitespace();
    let name = iter.next().unwrap().to_string();
    let phone = iter.next().unwrap().to_string();

    (name, phone)
}