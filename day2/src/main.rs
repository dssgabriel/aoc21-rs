#[derive(Debug)]
enum Movement {
    Forward(usize),
    Up(usize),
    Down(usize),
}

impl Movement {
    pub fn new(command: &str) -> Self {
        let command: Vec<&str> = command.splitn(2, " ").collect();
        let dir = command[0];
        let val: usize = command[1].parse().unwrap();

        match dir {
            "forward" => Movement::Forward(val),
            "up" => Movement::Up(val),
            "down" => Movement::Down(val),
            _ => panic!("wrong command"),
        }
    }
}

#[derive(Debug)]
struct Submarine {
    x: usize,
    depth: usize,
    aim: usize,
}

impl Submarine {
    pub fn new() -> Self {
        Submarine { x: 0, depth: 0, aim: 0 }
    }
        
    pub fn move_part_one(&mut self, movement: Movement) {
        match movement {
            Movement::Forward(v) => self.x += v,
            Movement::Up(v) => self.depth -= v,
            Movement::Down(v) => self.depth += v,
        }
    }

    pub fn move_part_two(&mut self, movement: Movement) {
        match movement {
            Movement::Forward(v) => {
                self.x += v;
                self.depth += self.aim * v;
            },
            Movement::Up(v) => self.aim -= v,
            Movement::Down(v) => self.aim += v,
        }
    }
}

fn part_one(input: &[String]) -> usize {
    let mut submarine = Submarine::new();

    for cmd in input {
        submarine.move_part_one(Movement::new(cmd));
    }

    submarine.x * submarine.depth
}

fn part_two(input: &[String]) -> usize {
    let mut submarine = Submarine::new();

    for cmd in input {
        submarine.move_part_two(Movement::new(cmd));
    }

    submarine.x * submarine.depth
}

fn main() {
    let input: Vec<String> = include_str!("../input/day2.txt")
        .trim()
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    println!("Answer to part 1 is {}", part_one(&input));
    println!("Answer to part 2 is {}", part_two(&input));
}
