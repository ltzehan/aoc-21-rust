struct Board {
    b: Vec<Vec<u32>>,
    marked: Vec<Vec<bool>>,
}

fn transpose<T: Copy>(v: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    (0..v[0].len())
        .map(|x| v.iter().map(|z| z[x]).collect())
        .collect()
}

impl Board {
    fn new(v: &Vec<Vec<u32>>) -> Board {
        Board {
            b: v.clone(),
            marked: vec![vec![false; v[0].len()]; v.len()],
        }
    }

    fn mark(&mut self, n: u32) {
        for i in 0..self.b.len() {
            for j in 0..self.b[0].len() {
                if n == self.b[i][j] {
                    self.marked[i][j] = true;
                    return;
                }
            }
        }
    }

    fn is_done(&self) -> bool {
        for row in &self.marked {
            if row.into_iter().all(|&x| x) {
                return true;
            }
        }
        for col in &transpose(&self.marked) {
            if col.into_iter().all(|&x| x) {
                return true;
            }
        }

        return false;
    }

    fn unmarked_sum(&self) -> u32 {
        self.b
            .iter()
            .flatten()
            .zip(self.marked.iter().flatten())
            .fold(0, |s, (&v, &marked)| if !marked { s + v } else { s })
    }
}

fn read_input() -> (Vec<u32>, Vec<Board>) {
    let ss = include_str!("in.txt").split_once("\n").unwrap();
    let nums =
        ss.0.split(",")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<u32>>();

    let boards =
        ss.1.lines()
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>()
            .chunks(5)
            .map(|board| {
                Board::new(
                    &board
                        .iter()
                        .map(|line| {
                            line.split_whitespace()
                                .map(|x| x.parse().unwrap())
                                .collect()
                        })
                        .collect(),
                )
            })
            .collect();

    (nums, boards)
}

fn first_puzzle() -> u32 {
    let (nums, mut boards) = read_input();
    for n in nums {
        for b in boards.iter_mut() {
            b.mark(n);
            if b.is_done() {
                return n * b.unmarked_sum();
            }
        }
    }

    return 0;
}

fn second_puzzle() -> u32 {
    let (nums, mut boards) = read_input();
    let mut done = vec![false; boards.len()];
    let mut count = boards.len();

    for n in nums {
        for (idx, b) in boards.iter_mut().enumerate() {
            b.mark(n);
            if !done[idx] && b.is_done() {
                if count == 1 {
                    return n * b.unmarked_sum();
                }
                count -= 1;
                done[idx] = true;
            }
        }
    }

    return 0;
}

#[test]
fn test_first_puzzle() {
    println!("Day 4 (1): {}", first_puzzle())
}

#[test]
fn test_second_puzzle() {
    println!("Day 4 (2): {}", second_puzzle());
}
