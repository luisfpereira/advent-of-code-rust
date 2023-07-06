use std::{fmt::Debug, fs};

trait Problem {
    fn char_to_shape(&self, value: char) -> Shape;
    fn parse(&self, file_path: &str) -> Vec<[Shape; 2]>;
}

struct Part1;
struct Part2;

impl Problem for Part1 {
    fn char_to_shape(&self, value: char) -> Shape {
        match value {
            'A' | 'X' => Shape::Rock,
            'B' | 'Y' => Shape::Paper,
            'C' | 'Z' => Shape::Scissors,
            _ => panic!("Unknown shape."),
        }
    }

    fn parse(&self, file_path: &str) -> Vec<[Shape; 2]> {
        fs::read_to_string(file_path)
            .unwrap()
            .lines()
            .map(|line| {
                line.trim()
                    .split(' ')
                    .map(|string| self.char_to_shape(string.chars().next().unwrap()))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect()
    }
}

impl Problem for Part2 {
    fn char_to_shape(&self, value: char) -> Shape {
        match value {
            'A' => Shape::Rock,
            'B' => Shape::Paper,
            'C' => Shape::Scissors,
            _ => panic!("Unknown shape."),
        }
    }

    fn parse(&self, file_path: &str) -> Vec<[Shape; 2]> {
        let contents = fs::read_to_string(file_path).unwrap();

        let mut strategy: Vec<[Shape; 2]> = Vec::new();
        for line in contents.lines() {
            let chars: Vec<char> = line
                .trim()
                .split(" ")
                .map(|x| x.chars().next().unwrap())
                .collect();
            let shape = self.char_to_shape(chars[0]);
            let result = self.char_to_result(chars[1]);

            let other_shape = shape.choose_for_outcome(result);

            strategy.push([shape, other_shape]);
        }
        strategy
    }
}

impl Part2 {
    fn char_to_result(&self, value: char) -> FightResult {
        match value {
            'X' => FightResult::Defeat,
            'Y' => FightResult::Draw,
            'Z' => FightResult::Victory,
            _ => panic!("Unknown result."),
        }
    }
}

#[derive(Eq, PartialEq)]
enum FightResult {
    Defeat,
    Draw,
    Victory,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

trait Score {
    fn score(&self) -> usize;
}

impl Score for Shape {
    fn score(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

impl Score for FightResult {
    fn score(&self) -> usize {
        match self {
            FightResult::Victory => 6,
            FightResult::Draw => 3,
            FightResult::Defeat => 0,
        }
    }
}

impl Shape {
    fn fight(&self, other_shape: &Shape) -> FightResult {
        if self == other_shape {
            return FightResult::Draw;
        };
        match (self, other_shape) {
            (Shape::Rock, Shape::Scissors)
            | (Shape::Scissors, Shape::Paper)
            | (Shape::Paper, Shape::Rock) => FightResult::Victory,
            _ => FightResult::Defeat,
        }
    }

    fn choose_for_outcome(&self, result: FightResult) -> Shape {
        // Retrieve shape that achieves given result against self
        match (self, result) {
            (Shape::Rock, FightResult::Victory) => Shape::Paper,
            (Shape::Rock, FightResult::Defeat) => Shape::Scissors,
            (Shape::Paper, FightResult::Victory) => Shape::Scissors,
            (Shape::Paper, FightResult::Defeat) => Shape::Rock,
            (Shape::Scissors, FightResult::Victory) => Shape::Rock,
            (Shape::Scissors, FightResult::Defeat) => Shape::Paper,
            _ => self.clone(),
        }
    }
}

fn score_fight(shape: &Shape, other_shape: &Shape) -> usize {
    shape.fight(&other_shape).score() + shape.score()
}

fn solve(problem: &impl Problem, file_path: &str) -> usize {
    let strategy = problem.parse(file_path);

    strategy
        .iter()
        .map(|shapes| score_fight(&shapes[1], &shapes[0]))
        .sum()
}

pub fn solve_part1(file_path: &str) -> usize {
    let problem = Part1 {};
    solve(&problem, file_path)
}

pub fn solve_part2(file_path: &str) -> usize {
    let problem = Part2 {};
    solve(&problem, file_path)
}
