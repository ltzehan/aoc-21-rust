use std::collections::HashSet;
use std::iter::FromIterator;

fn first_puzzle() -> u32 {
    let outputs: Vec<_> = include_str!("in.txt")
        .lines()
        .map(|l| l.trim().split_once("|").unwrap().1)
        .map(|s| s.split(" "))
        .flatten()
        .collect();

    outputs
        .iter()
        .filter(|x| x.len() == 2 || x.len() == 3 || x.len() == 4 || x.len() == 7)
        .count() as u32
}

// Segment labels:
//   0000
//  1    2
//  1    2
//   3333
//  4    5
//  4    5
//   6666

fn second_puzzle() -> u32 {
    let inputs: Vec<_> = include_str!("in.txt")
        .lines()
        .map(|l| l.trim().split_once("|").unwrap())
        .collect();

    inputs
        .iter()
        .map(|(l, out)| {
            let digits: Vec<_> = l.split(" ").collect();
            // Known digits from length
            let mut dstr = vec![HashSet::new(); 10];
            digits.iter().for_each(|d| match d.len() {
                2 => dstr[1] = HashSet::from_iter(d.chars()),
                3 => dstr[7] = HashSet::from_iter(d.chars()),
                4 => dstr[4] = HashSet::from_iter(d.chars()),
                7 => dstr[8] = HashSet::from_iter(d.chars()),
                _ => (),
            });

            let len6: Vec<_> = digits.iter().filter(|d| d.len() == 6).collect();
            len6.iter().for_each(|d| {
                let h = HashSet::from_iter(d.chars());
                if !h.is_superset(&dstr[7]) {
                    dstr[6] = h;
                } else {
                    if h.is_superset(&dstr[4]) {
                        dstr[9] = h;
                    } else {
                        dstr[0] = h;
                    }
                }
            });

            // Find segment 2
            let seg2 = *dstr[1].difference(&dstr[6]).collect::<Vec<&char>>()[0];
            let len5: Vec<_> = digits.iter().filter(|d| d.len() == 5).collect();
            len5.iter().for_each(|d| {
                let h = HashSet::from_iter(d.chars());
                if h.is_superset(&dstr[7]) {
                    dstr[3] = h;
                } else {
                    if h.contains(&seg2) {
                        dstr[2] = h;
                    } else {
                        dstr[5] = h;
                    }
                }
            });

            out.trim()
                .split(" ")
                .map(|d| {
                    let dh = HashSet::from_iter(d.chars());
                    dstr.iter()
                        .position(|h| h.symmetric_difference(&dh).count() == 0)
                        .unwrap()
                })
                .fold(0, |s, x| s * 10 + x as u32)
        })
        .fold(0, |s, x| s + x)
}

#[test]
fn test_first_puzzle() {
    println!("Day 8 (1): {}", first_puzzle())
}

#[test]
fn test_second_puzzle() {
    println!("Day 8 (2): {}", second_puzzle());
}
