fn is_in(grid: &Vec<Vec<u32>>, i: i32, j: i32) -> bool {
    if i < 0 || i >= grid.len() as i32 || j < 0 || j >= grid[0].len() as i32 {
        return false;
    }

    return true;
}

fn update(
    grid: &mut Vec<Vec<u32>>,
    flashed: &mut Vec<Vec<bool>>,
    i: usize,
    j: usize,
    count: &mut u32,
) {
    grid[i][j] += 1;
    if grid[i][j] > 9 && !flashed[i][j] {
        flashed[i][j] = true;
        *count += 1;

        // Update surrounding cells
        for dx in -1_i32..=1 {
            for dy in -1_i32..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                // this is so ugly
                let x = i as i32 + dx;
                let y = j as i32 + dy;
                if is_in(grid, x, y) {
                    update(grid, flashed, x as usize, y as usize, count);
                }
            }
        }
    }
}

fn first_puzzle() {
    let mut grid: Vec<Vec<u32>> = include_str!("in.txt")
        .lines()
        .map(|l| l.chars().map(|c| char::to_digit(c, 10).unwrap()).collect())
        .collect();

    let mut count = 0;
    let mut step = 1;
    loop {
        let mut flashed = vec![vec![false; grid[0].len()]; grid.len()];
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                update(&mut grid, &mut flashed, i, j, &mut count);
            }
        }

        let mut all_flashed = true;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] > 9 {
                    grid[i][j] = 0;
                } else {
                    all_flashed = false;
                }
            }
        }

        if step == 100 {
            println!("Day 11 (1): {}", count);
        }
        if all_flashed {
            println!("Day 11 (2): {}", step);
            break;
        }

        step += 1;
    }
}

#[test]
fn test_puzzle() {
    first_puzzle();
}
