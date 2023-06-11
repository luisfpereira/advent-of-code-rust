use std::{fmt::Debug, fs};

enum FightResult {
    Defeat,
    Draw,
    Victory,
}

#[derive(Debug, Eq, PartialEq)]
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

impl From<char> for Shape {
    fn from(value: char) -> Self {
        match value {
            'A' | 'X' => Shape::Rock,
            'B' | 'Y' => Shape::Paper,
            'C' | 'Z' => Shape::Scissors,
            _ => panic!("Unknown shape."),
        }
    }
}

trait Duel {
    fn fight(&self, other_shape: &Shape) -> FightResult;
}

impl Duel for Shape {
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
}

fn score_fight(shape: &Shape, other_shape: &Shape) -> usize {
    shape.fight(&other_shape).score() + shape.score()
}

fn parse(file_path: &str) -> Vec<[Shape; 2]> {
    fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(|line| {
            line.trim()
                .split(' ')
                .map(|string| Shape::from(string.chars().next().unwrap()))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect()
}

pub fn solve_part1(file_path: &str) -> usize {
    let strategy = parse(file_path);

    strategy
        .iter()
        .map(|shapes| score_fight(&shapes[1], &shapes[0]))
        .sum()
}
