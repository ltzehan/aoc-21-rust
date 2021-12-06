use std::collections::HashMap;
use std::mem::swap;

type Point = (i32, i32);
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn new(ss: &'static str) -> Line {
        let p = ss.split_once(" -> ").unwrap();
        fn parse_pos(pos: &'static str) -> Point {
            let (x, y) = pos.split_once(",").unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        }

        Line {
            start: parse_pos(p.0),
            end: parse_pos(p.1),
        }
    }

    fn is_vertical(&self) -> bool {
        self.start.0 == self.end.0
    }
    fn is_horizontal(&self) -> bool {
        self.start.1 == self.end.1
    }
    fn is_diagonal(&self) -> bool {
        (self.start.1 - self.end.1).abs() == (self.start.0 - self.end.0).abs()
    }

    fn get_points(&self) -> Vec<Point> {
        if self.is_horizontal() {
            let (mut a, mut b) = (self.start.0, self.end.0);
            if a > b {
                swap(&mut a, &mut b);
            }
            return (a..=b).map(|x| (x, self.start.1)).collect();
        } else if self.is_vertical() {
            let (mut a, mut b) = (self.start.1, self.end.1);
            if a > b {
                swap(&mut a, &mut b);
            }
            return (a..=b).map(|y| (self.start.0, y)).collect();
        } else if self.is_diagonal() {
            let (mut a, mut b) = (self.start, self.end);
            if a.0 > b.0 {
                swap(&mut a, &mut b);
            }
            let dir = (b.1 - a.1).signum();
            return (a.0..=b.0)
                .enumerate()
                .map(|(i, x)| (x, a.1 + dir * (i as i32)))
                .collect();
        }

        vec![]
    }
}

fn first_puzzle() -> u32 {
    let points: Vec<_> = include_str!("in.txt")
        .lines()
        .map(|x| Line::new(x))
        .filter(|x| x.is_horizontal() || x.is_vertical())
        .map(|x| x.get_points())
        .flatten()
        .collect();

    let mut count = HashMap::new();
    points.iter().for_each(|p| {
        *count.entry(p).or_insert(0) += 1;
    });

    count.into_iter().filter(|&(_, c)| c >= 2).count() as u32
}

fn second_puzzle() -> u32 {
    let points: Vec<_> = include_str!("in.txt")
        .lines()
        .map(|x| Line::new(x))
        .filter(|x| x.is_horizontal() || x.is_vertical() || x.is_diagonal())
        .map(|x| x.get_points())
        .flatten()
        .collect();

    let mut count = HashMap::new();
    points.iter().for_each(|p| {
        *count.entry(p).or_insert(0) += 1;
    });

    count.into_iter().filter(|&(_, c)| c >= 2).count() as u32
}

#[test]
fn test_first_puzzle() {
    println!("Day 5 (1): {}", first_puzzle())
}

#[test]
fn test_second_puzzle() {
    println!("Day 5 (2): {}", second_puzzle());
}
