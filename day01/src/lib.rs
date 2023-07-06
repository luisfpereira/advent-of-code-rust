use std::{fs, ops::Add};

fn parse(file_path: &str) -> Vec<Vec<usize>> {
    let contents = fs::read_to_string(file_path).unwrap();

    contents
        .split("\n\n")
        .map(|x| {
            x.lines()
                .map(|y| y.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn sum_elf_calories<T>(elf_calories: &[T], init: T) -> T
where
    T: Copy + Add<T, Output = T>,
{
    elf_calories
        .iter()
        .fold(init, |calories, &item| calories + item)
}

pub fn solve_part1(file_path: &str) -> usize {
    let elves_calories = parse(file_path);

    let elves_sum: Vec<usize> = elves_calories
        .iter()
        .map(|elf_calories| sum_elf_calories(&elf_calories, 0 as usize))
        .collect();

    let max = elves_sum.iter().max().unwrap();

    *max
}

pub fn solve_part2(file_path: &str) -> usize {
    let elves_calories = parse(file_path);

    let mut elves_sum: Vec<usize> = elves_calories
        .iter()
        .map(|elf_calories| sum_elf_calories(elf_calories, 0 as usize))
        .collect();

    elves_sum.sort();

    elves_sum.iter().rev().take(3).sum()
}
