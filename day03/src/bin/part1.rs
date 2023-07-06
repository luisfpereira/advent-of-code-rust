use day03::solve_part1 as solve;

fn main() {
    let file_path = "input/input.dat";

    let priorities = solve(file_path);

    println!(
        "What is the sum of the priorities of those item types? {}",
        priorities
    );
}

#[test]
fn example() {
    let file_path = "input/input_example.dat";
    let priorities = solve(file_path);
    assert_eq!(priorities, 157);
}
