use day01::solve_part2 as solve;

fn main() {
    let file_path = "input/input.dat";

    let total_calories = solve(file_path);

    println!(
        "How many Calories are those Elves carrying in total? {}",
        total_calories
    );
}

#[test]
fn part2() {
    let file_path = "input/input_example.dat";
    let total_calories = solve(file_path);
    assert_eq!(total_calories, 45000);
}
