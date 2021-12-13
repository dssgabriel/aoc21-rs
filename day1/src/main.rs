use std::include_str;

fn part_one(depths: &[i32]) -> u32 {
    let mut count = 0;
    let mut prev = &depths[0];

    for curr in depths.iter().skip(1) {
        if curr > prev {
            count += 1;
        }
        prev = curr;
    }

    count
}

fn part_two(depths: &[i32]) -> u32 {
    let aggregate: Vec<i32> = depths.windows(3).map(|s| s.iter().sum()).collect();
    let count = part_one(&aggregate);

    count
}

fn main() {
    let depths: Vec<i32> = include_str!("day1.in")
        .trim()
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    println!("Answer of part 1 is {}", part_one(&depths));
    println!("Answer of part 2 is {}", part_two(&depths));
}
