fn main() {
    let t: i32 = read_t();

    for _ in 0..t {
        let mut left = String::new();
        let mut right = String::new();

        let str = read_str();

        for (i, c) in str.trim().chars().enumerate() {
            if i % 2 == 0 {
                left.push(c)
            } else {
                right.push(c)
            }
        }

        println!("{} {}", left, right);
    }
}

fn read_t() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn read_str() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}
