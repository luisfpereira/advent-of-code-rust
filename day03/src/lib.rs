use std::fs;

pub fn parse(file_path: &str) -> Vec<[String; 2]> {
    let contents = fs::read_to_string(file_path).unwrap();

    let mut rucksacks: Vec<[String; 2]> = Vec::new();
    for line in contents.lines() {
        let (half1, half2) = line.split_at(line.len() / 2);
        rucksacks.push([half1.to_owned(), half2.to_owned()]);
    }

    rucksacks
}

pub fn get_repeated(rucksack: [String; 2]) -> char {
    for item in rucksack[0].chars() {
        for other_item in rucksack[1].chars() {
            if other_item == item {
                return item;
            }
        }
    }
    panic!("Cannot find repeated item: {:?}", rucksack)
}

pub fn get_priority(item: char) -> usize {
    let alphabet: String = ('a'..='z').collect();

    match alphabet.find(item) {
        Some(x) => x + 1,
        None => 1 + alphabet.len() + alphabet.to_uppercase().find(item).unwrap(),
    }
}

pub fn solve_part1(file_path: &str) -> usize {
    let rucksacks = parse(file_path);
    let repeated: Vec<char> = rucksacks
        .into_iter()
        .map(|rucksack| get_repeated(rucksack))
        .collect();

    repeated.iter().map(|&item| get_priority(item)).sum()
}
