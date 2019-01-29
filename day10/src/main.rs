use std::cmp;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let mut n = input.trim().parse::<i32>().unwrap();

    let mut result = 0;
    let mut count = 0;
    while n != 0 {
        if n & 1 == 1 {
            count += 1;
        } else {
            result = cmp::max(result, count);
            count = 0;
        }

        n >>= 1;
    }

    result = cmp::max(result, count);
    println!("{}", result);
}
