use std::include_str;

fn part_one(input: &[i32]) -> u32 {
    input.windows(2).map(|d| (d[0] < d[1]) as u32).sum()
}

fn part_two(input: &[i32]) -> u32 {
    let aggregate: Vec<i32> = input.windows(3).map(|s| s.iter().sum()).collect();
    part_one(&aggregate)
}

fn main() {
    let input: Vec<i32> = include_str!("../input/day1.txt")
        .trim()
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    println!("Answer to part 1 is {}", part_one(&input));
    println!("Answer to part 2 is {}", part_two(&input));
}
