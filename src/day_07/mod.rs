// n-th triangular number
fn tri(n: u32) -> u32 {
    n * (n + 1) / 2
}

fn first_puzzle() -> u32 {
    let mut crabs: Vec<i32> = include_str!("in.txt")
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    let n = crabs.len();
    crabs.sort();
    let pos = if n % 2 == 1 {
        crabs[n / 2]
    } else {
        (crabs[n / 2] + crabs[n / 2 - 1]) / 2
    };

    crabs.iter().fold(0, |s, x| s + (*x - pos).abs() as u32)
}

fn second_puzzle() -> u32 {
    let crabs: Vec<i32> = include_str!("in.txt")
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    let (a, b) = (*crabs.iter().min().unwrap(), *crabs.iter().max().unwrap());
    (a..b)
        .map(|pos| -> u32 {
            crabs
                .iter()
                .fold(0, |s, x| s + tri((pos - *x).abs() as u32))
        })
        .min()
        .unwrap()
}

#[test]
fn test_first_puzzle() {
    println!("Day 7 (1): {}", first_puzzle())
}

#[test]
fn test_second_puzzle() {
    println!("Day 7 (2): {}", second_puzzle());
}
