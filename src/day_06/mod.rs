use std::collections::HashMap;

fn simulate(days: u32) -> u64 {
    let mut fish = HashMap::new();
    include_str!("in.txt")
        .split(",")
        .map(|s| s.parse().unwrap())
        .for_each(|x: u32| *fish.entry(x).or_insert(0) += 1);

    (0..days).for_each(|_| {
        let mut next: HashMap<u32, u64> = HashMap::new();
        for i in 1..=8 {
            next.insert(i - 1, *fish.get(&i).unwrap_or(&0));
        }
        let n = *fish.get(&0).unwrap_or(&0);
        next.insert(8, n);
        next.entry(6).and_modify(|x| *x += n);

        fish = next;
    });

    fish.values().fold(0, |s, x| s + *x as u64)
}

fn first_puzzle() -> u64 {
    simulate(80)
}

fn second_puzzle() -> u64 {
    simulate(256)
}

#[test]
fn test_first_puzzle() {
    println!("Day 6 (1): {}", first_puzzle())
}

#[test]
fn test_second_puzzle() {
    println!("Day 6 (2): {}", second_puzzle());
}
