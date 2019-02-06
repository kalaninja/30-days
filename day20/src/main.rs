fn main() {
    let stdin = std::io::stdin();

    let mut n_string = String::new();
    stdin.read_line(&mut n_string).unwrap();
    let n: usize = n_string.trim().parse().unwrap();

    let mut arr_string = String::new();
    stdin.read_line(&mut arr_string).unwrap();

    let mut arr = vec![0; n];
    arr_string.trim().split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .zip(arr.iter_mut())
        .for_each(|(n, a)| *a = n);

    let swaps = bubble_sort(&mut *arr);
    println!("Array is sorted in {} swaps.", swaps);
    println!("First Element: {}", arr[0]);
    println!("Last Element: {}", arr[arr.len() - 1]);
}

fn bubble_sort(arr: &mut [i32]) -> i32 {
    let mut result = 0;
    let length = arr.len();

    for _ in 0..length {
        let mut number_of_swaps = 0;

        for j in 0..length - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                number_of_swaps += 1;
            }
        }

        if number_of_swaps == 0 { break; } else { result += number_of_swaps; }
    }

    result
}
