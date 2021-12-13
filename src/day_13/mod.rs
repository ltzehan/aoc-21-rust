use std::collections::HashSet;

enum FoldDirection {
    Horizontal,
    Vertical,
}
struct FoldLine {
    dir: FoldDirection,
    at: usize,
}

impl FoldLine {
    fn new(s: &str) -> Self {
        match s.split_once("=").unwrap() {
            ("fold along x", n) => FoldLine {
                dir: FoldDirection::Vertical,
                at: n.parse().unwrap(),
            },
            ("fold along y", n) => FoldLine {
                dir: FoldDirection::Horizontal,
                at: n.parse().unwrap(),
            },
            (_, _) => unreachable!(),
        }
    }
}

fn parse_input() -> (HashSet<(usize, usize)>, Vec<FoldLine>) {
    let (dots, folds) = include_str!("in.txt").split_once("\n\n").unwrap();

    (
        dots.lines()
            .map(|l| {
                let (x, y) = l.split_once(",").unwrap();
                (x.parse().unwrap(), y.parse().unwrap())
            })
            .collect(),
        folds.lines().map(|l| FoldLine::new(l)).collect(),
    )
}

fn first_puzzle() {
    let (mut dots, folds) = parse_input();
    for (idx, fold) in folds.iter().enumerate() {
        dots = dots
            .iter()
            .map(|&(x, y)| match fold {
                FoldLine {
                    dir: FoldDirection::Horizontal,
                    at: n,
                } => {
                    if y > *n {
                        return (x, 2 * n - y);
                    } else {
                        return (x, y);
                    }
                }
                FoldLine {
                    dir: FoldDirection::Vertical,
                    at: n,
                } => {
                    if x > *n {
                        return (2 * n - x, y);
                    } else {
                        return (x, y);
                    }
                }
            })
            .collect();

        if idx == 0 {
            println!("Day 13 (1): {}", dots.len())
        }
    }

    let (xl, yl): (Vec<_>, Vec<_>) = dots.iter().cloned().unzip();
    let mut grid = vec![vec!['.'; *yl.iter().max().unwrap() + 1]; *xl.iter().max().unwrap() + 1];

    for (x, y) in dots {
        grid[x][y] = '#';
    }
    for i in 0..grid[0].len() {
        for j in 0..grid.len() {
            print!("{}", grid[j][i]);
        }
        println!();
    }
}

#[test]
fn test_puzzle() {
    first_puzzle()
}
