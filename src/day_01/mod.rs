fn first_puzzle() -> u32 {
    include_str!("in.txt")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u32>>()
        .windows(2)
        .fold(0, |s, x| if x[1] > x[0] { s + 1 } else { s })
}

fn second_puzzle() -> u32 {
    include_str!("in.txt")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u32>>()
        .windows(3)
        .map(|x| x.iter().sum())
        .collect::<Vec<u32>>()
        .windows(2)
        .fold(0, |s, x| if x[1] > x[0] { s + 1 } else { s })
}

#[test]
fn test_first_puzzle() {
    println!("Day 1: {}", first_puzzle())
}

#[test]
fn test_second_puzzle() {
    println!("Day 2: {}", second_puzzle());
}
