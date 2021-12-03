use std::convert::TryInto;

fn has_more_or_eq(v: &Vec<char>, c: char) -> bool {
    v.iter().filter(|x| **x == c).count() >= (v.len() + 1) / 2
}

fn first_puzzle() -> u32 {
    let s: Vec<Vec<_>> = include_str!("in.txt")
        .lines()
        .map(|x| x.chars().collect())
        .collect();

    let n = s[0].len();
    let bstr = (0..n)
        // Transpose
        .map(|i| s.iter().map(|x| x[i]).collect())
        .map(|l| has_more_or_eq(&l, '1'))
        .map(|x| if x { '1' } else { '0' })
        .collect::<String>();

    let gamma = u32::from_str_radix(&bstr, 2).unwrap();
    let epsilon = (u32::pow(2, n.try_into().unwrap()) - 1) ^ gamma;

    gamma * epsilon
}

fn second_puzzle() -> u32 {
    let s: Vec<Vec<_>> = include_str!("in.txt")
        .lines()
        .map(|x| x.chars().collect())
        .collect();

    let n = s[0].len();
    let mut sl = s.clone();
    for i in 0..n {
        let bits = sl.iter().map(|x| x[i]).collect();
        let freqb = if has_more_or_eq(&bits, '1') { '1' } else { '0' };
        sl.retain(|x| x[i] == freqb);
        if sl.len() == 1 {
            break;
        }
    }
    let oxygen = u32::from_str_radix(&sl[0].iter().collect::<String>(), 2).unwrap();

    let mut sl = s.clone();
    for i in 0..n {
        let bits = sl.iter().map(|x| x[i]).collect();
        let freqb = if has_more_or_eq(&bits, '1') { '0' } else { '1' };
        sl.retain(|x| x[i] == freqb);
        if sl.len() == 1 {
            break;
        }
    }
    let co2 = u32::from_str_radix(&sl[0].iter().collect::<String>(), 2).unwrap();

    oxygen * co2
}

#[test]
fn test_first_puzzle() {
    println!("Day 3 (1): {}", first_puzzle())
}

#[test]
fn test_second_puzzle() {
    println!("Day 3 (2): {}", second_puzzle());
}
