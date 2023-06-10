use std::fs;

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

fn sum_calories(elves_calories: &[Vec<usize>]) -> Vec<usize> {
    elves_calories.iter().map(|x| x.iter().sum()).collect()
}

pub fn solve_part1(file_path: &str) -> usize {
    let elves_calories = parse(file_path);

    let elves_sum = sum_calories(&elves_calories);

    let max = elves_sum.iter().max().unwrap();

    *max
}

pub fn solve_part2(file_path: &str) -> usize {
    let elves_calories = parse(file_path);

    let mut elves_sum = sum_calories(&elves_calories);
    elves_sum.sort();

    elves_sum.iter().rev().take(3).sum()
}

#[test]
fn part1() {
    let file_path = "input/input_example.dat";
    let max = solve_part1(file_path);
    assert_eq!(max, 24000);
}

#[test]
fn part2() {
    let file_path = "input/input_example.dat";
    let total_calories = solve_part2(file_path);
    assert_eq!(total_calories, 45000);
}
