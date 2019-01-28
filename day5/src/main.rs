fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let n: i32 = input.trim().parse().unwrap();
    for i in 1..=10 {
        println!("{} x {} = {}", n, i, n * i);
    }
}
