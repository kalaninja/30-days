const N: usize = 6;

fn main() {
    let mut a = [[0; N]; N];

    for i in 0..N {
        let line = read_ln();
        let mut values = line.trim().split_whitespace();

        for j in 0..N {
            a[i][j] = values.next().unwrap().parse().unwrap();
        }
    }

    let mut result = std::i32::MIN;
    for i in 0..N - 2 {
        for j in 0..N - 2 {
            result = std::cmp::max(hourglass_sum(&a, (i, j)), result);
        }
    }

    println!("{}", result);
}

fn read_ln() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

fn hourglass_sum(a: &[[i32; N]; N], start: (usize, usize)) -> i32 {
    let (i, j) = start;

    a[i][j] + a[i][j + 1] + a[i][j + 2]
        + a[i + 1][j + 1]
        + a[i + 2][j] + a[i + 2][j + 1] + a[i + 2][j + 2]
}
