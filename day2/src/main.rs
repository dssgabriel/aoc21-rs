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

fn part_one(commands: &[String]) -> usize {
    let mut submarine = Submarine::new();

    for command in commands {
        submarine.move_part_one(Movement::new(command));
    }

    submarine.x * submarine.depth
}

fn part_two(commands: &[String]) -> usize {
    let mut submarine = Submarine::new();

    for command in commands {
        submarine.move_part_two(Movement::new(command));
    }

    submarine.x * submarine.depth
}

fn main() {
    let commands: Vec<String> = include_str!("day2.in")
        .trim()
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    println!("Answer of part 1 is {}", part_one(&commands));
    println!("Answer of part 2 is {}", part_two(&commands));
}
