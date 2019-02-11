fn main() {
    let t: i32 = read_num();

    for _ in 0..t {
        let n = read_num();

        if is_prime(n) {
            println!("Prime")
        } else {
            println!("Not prime")
        }
    }
}

fn read_num() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn is_prime(n: i32) -> bool {
    if n <= 1 {
        false
    } else if n <= 3 {
        true
    } else if n % 2 == 0 || n % 3 == 0 {
        false
    } else {
        let limit = (n as f32).sqrt() as i32;
        (5..=limit).step_by(6).all(|x| n % x != 0 && n % (x + 2) != 0)
    }
}
