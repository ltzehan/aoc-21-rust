fn first_puzzle() -> u32 {
    let nums: Vec<Vec<u32>> = include_str!("in.txt")
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut peaks = vec![];
    for i in 0..nums.len() {
        for j in 0..nums[0].len() {
            let curr = nums[i][j];
            if (i > 0 && curr >= nums[i - 1][j])
                || (i < nums.len() - 1 && curr >= nums[i + 1][j])
                || (j > 0 && curr >= nums[i][j - 1])
                || (j < nums[0].len() - 1 && curr >= nums[i][j + 1])
            {
                continue;
            }

            peaks.push(nums[i][j]);
        }
    }

    peaks.iter().fold(0, |s, x| s + x + 1)
}

fn flood_fill(
    v: &Vec<Vec<u32>>,
    seen: &mut Vec<Vec<bool>>,
    count: &mut Vec<u32>,
    i: usize,
    j: usize,
    cc: usize,
) {
    if seen[i][j] || v[i][j] == 9 {
        return;
    }

    seen[i][j] = true;
    count[cc] += 1;

    if i > 0 {
        flood_fill(v, seen, count, i - 1, j, cc);
    }
    if i < seen.len() - 1 {
        flood_fill(v, seen, count, i + 1, j, cc);
    }
    if j > 0 {
        flood_fill(v, seen, count, i, j - 1, cc);
    }
    if j < seen[0].len() - 1 {
        flood_fill(v, seen, count, i, j + 1, cc);
    }
}

fn second_puzzle() -> u32 {
    let nums: Vec<Vec<u32>> = include_str!("in.txt")
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut seen = vec![vec![false; nums[0].len()]; nums.len()];
    let mut count = vec![];
    for i in 0..nums.len() {
        for j in 0..nums[0].len() {
            if !seen[i][j] && nums[i][j] != 9 {
                count.push(0);
                let cc = count.len() - 1;
                flood_fill(&nums, &mut seen, &mut count, i, j, cc);
            }
        }
    }

    count.sort();
    count[count.len() - 3..].iter().fold(1, |s, x| s * x)
}

#[test]
fn test_first_puzzle() {
    println!("Day 9 (1): {}", first_puzzle())
}

#[test]
fn test_second_puzzle() {
    println!("Day 9 (2): {}", second_puzzle());
}
