fn main() {
    let actual = read_dt();
    let expected = read_dt();

    print!("{}", calculate_fine(actual, expected))
}

fn calculate_fine(actual: Date, expected: Date) -> i32 {
    match actual.year - expected.year {
        y if y > 0 => 10000,
        y if y < 0 => 0,
        _ => match actual.month - expected.month {
            m if m > 0 => 500 * m,
            m if m < 0 => 0,
            _ => if actual.day > expected.day { 15 * (actual.day - expected.day) } else { 0 }
        }
    }
}

#[test]
fn test_calculate_fine() {
    let actual = Date { day: 3, month: 7, year: 2014 };
    let expected = Date { day: 6, month: 6, year: 2015 };
    let fine = calculate_fine(actual, expected);

    assert_eq!(fine, 0);
}

fn read_dt() -> Date {
    match read_ln().trim().split_whitespace()
        .map(|x| -> i32 { x.trim().parse().unwrap() })
        .collect::<Vec<_>>().as_slice() {
        &[day, month, year] => Date { day, month, year },
        _ => unreachable!()
    }
}

fn read_ln() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

#[derive(Debug)]
struct Date {
    day: i32,
    month: i32,
    year: i32,
}