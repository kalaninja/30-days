fn main() {
    let t = read_ln().trim().parse().unwrap();
    for _ in 0..t {
        let (n, k) = read_params();
        println!("{}", calculate(n, k));
    }
}

fn calculate(n: i32, k: i32) -> i32 {
    (1..n).flat_map(|a| (a + 1..=n).map(move |b| a & b))
        .fold(0, |acc, x| if x < k && x > acc { x } else { acc })
}

#[test]
fn test_calculate() {
    let result = calculate(5, 2);

    assert_eq!(result, 1);
}

fn read_params() -> (i32, i32) {
    let line = read_ln();
    let mut iter = line.trim().split_whitespace();
    let n = iter.next().unwrap().parse().unwrap();
    let k = iter.next().unwrap().parse().unwrap();
    (n, k)
}

fn read_ln() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}