use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let n: i32 = stdin.lock().lines().next().unwrap().unwrap().trim().parse().unwrap();

    println!("{}", factorial(n))
}

fn factorial(n: i32) -> i32 {
    if n <= 1 { 1 } else { n * factorial(n - 1) }
}
