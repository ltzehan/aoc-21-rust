use std::collections::HashMap;

fn first_puzzle() -> u32 {
    let value = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let end = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);

    include_str!("in.txt")
        .lines()
        .map(|l| {
            let mut stack: Vec<char> = vec![];
            for c in l.chars() {
                if value.get(&c).is_some() {
                    // Ending bracket
                    if stack[stack.len() - 1] == c {
                        stack.pop();
                    } else {
                        return *value.get(&c).unwrap();
                    }
                } else {
                    // Opening bracket
                    stack.push(*end.get(&c).unwrap());
                }
            }

            0
        })
        .fold(0, |s, x| s + x)
}

fn second_puzzle() -> u64 {
    let value = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);
    let end = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);

    let mut scores: Vec<_> = include_str!("in.txt")
        .lines()
        .map(|l| {
            let mut stack: Vec<char> = vec![];
            for c in l.chars() {
                if end.get(&c).is_none() {
                    // Ending bracket
                    if stack[stack.len() - 1] == c {
                        stack.pop();
                    } else {
                        // Corrupted line
                        return None;
                    }
                } else {
                    // Opening bracket
                    stack.push(*end.get(&c).unwrap());
                }
            }

            Some(
                stack
                    .iter()
                    .rev()
                    .map(|c| *value.get(&c).unwrap())
                    .fold(0, |s, x| s * 5 + x),
            )
        })
        .flat_map(|x| x)
        .collect();

    scores.sort();
    scores[scores.len() / 2]
}

#[test]
fn test_first_puzzle() {
    println!("Day 10 (1): {}", first_puzzle())
}

#[test]
fn test_second_puzzle() {
    println!("Day 10 (2): {}", second_puzzle());
}
