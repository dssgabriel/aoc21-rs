use std::include_str;
use itertools;

fn diagnostic(input: &str) -> (usize, Vec<u32>) {
    (
        input.lines().next().unwrap().len(),
        input
            .lines()
            .map(|l| u32::from_str_radix(l, 2).unwrap())
            .collect(),
    )
}

fn select(width: usize, numbers: &mut [u32], invert_selection: bool) -> u32 {
    if numbers.len() == 1 {
        return numbers[0];
    }

    let index = itertools::partition(&mut *numbers, |n| n & (1 << (width - 1)) != 0);

    if invert_selection ^ (index * 2 >= numbers.len()) {
        select(width - 1, &mut numbers[..index], invert_selection)
    } else {
        select(width - 1, &mut numbers[index..], invert_selection)
    }
}

fn part_one(input: &str) -> u32 {
    let (width, numbers) = diagnostic(input);

    let gamma: u32 = (0..width)
        .map(|i| 1u32 << i)
        .filter(|i| numbers.iter().filter(|n| *n & i != 0).count() * 2 > numbers.len())
        .sum();

    gamma * (((1u32 << width) - 1) ^ gamma)
}

fn part_two(input: &str) -> u32 {
    let (width, mut numbers) = diagnostic(input);
    select(width, &mut numbers, false) * select(width, &mut numbers, true)
}

fn main() {
    let input = include_str!("day3.in").trim();
    println!("Answer to part 1 is: {}", part_one(&input));
    println!("Answer to part 2 is: {}", part_two(&input));
}
