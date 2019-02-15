extern crate regex;

#[macro_use]
extern crate lazy_static;

use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^(?P<name>\w+)\s[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@gmail\.com$").unwrap();
}

fn main() {
    let n: i32 = read_ln().trim().parse().unwrap();

    let mut result = (0..n).map(|_| read_ln())
        .map(|x| parse_ln(x.trim()))
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();

    result.sort();
    for x in result {
        println!("{}", x)
    }
}

fn read_ln() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

fn parse_ln(entry: &str) -> Option<String> {
    RE.captures(entry).map_or(None, |caps| {
        caps.name("name").map(|x| x.as_str().to_string())
    })
}

#[test]
fn test_parse_ln() {
    let entry = r"riya riya@gmail.com";
    let result = parse_ln(entry);

    assert_eq!(result, Some(String::from(r"riya")));
}