enum Direction {
    Forward,
    Up,
    Down,
}
impl From<&'static str> for Direction {
    fn from(s: &'static str) -> Self {
        match s {
            "forward" => Direction::Forward,
            "up" => Direction::Up,
            "down" => Direction::Down,
            _ => unreachable!(),
        }
    }
}

struct Movement {
    dir: Direction,
    n: u32,
}
impl Movement {
    fn new(s: (&'static str, &'static str)) -> Movement {
        Movement {
            dir: Direction::from(s.0),
            n: s.1.parse::<u32>().unwrap(),
        }
    }
}

struct Pos {
    x: u32,
    y: u32,
    aim: u32,
}
impl Pos {
    fn move_simple(&mut self, mv: Movement) {
        match mv.dir {
            Direction::Forward => self.x += mv.n,
            Direction::Up => self.y -= mv.n,
            Direction::Down => self.y += mv.n,
        }
    }

    fn move_aim(&mut self, mv: Movement) {
        match mv.dir {
            Direction::Forward => {
                self.x += mv.n;
                self.y += self.aim * mv.n;
            }
            Direction::Up => self.aim -= mv.n,
            Direction::Down => self.aim += mv.n,
        }
    }
}

fn first_puzzle() -> u32 {
    let mut pos = Pos { x: 0, y: 0, aim: 0 };
    include_str!("in.txt")
        .lines()
        .map(|s| Movement::new(s.split_once(" ").unwrap()))
        .for_each(|s| pos.move_simple(s));

    return pos.x * pos.y;
}

fn second_puzzle() -> u32 {
    let mut pos = Pos { x: 0, y: 0, aim: 0 };
    include_str!("in.txt")
        .lines()
        .map(|s| Movement::new(s.split_once(" ").unwrap()))
        .for_each(|s| pos.move_aim(s));

    return pos.x * pos.y;
}

#[test]
fn test_first_puzzle() {
    println!("Day 2 (1): {}", first_puzzle())
}

#[test]
fn test_second_puzzle() {
    println!("Day 2 (2): {}", second_puzzle());
}
