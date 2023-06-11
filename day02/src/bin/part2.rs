use day02::solve_part2 as solve;

fn main() {
    let file_path = "input/input.dat";

    let score = solve(file_path);

    println!(
        "What would your total score be if everything goes exactly according to your strategy guide? {}",
        score
        );
}

#[test]
fn example() {
    let file_path = "input/input_example.dat";
    let score = solve(file_path);
    assert_eq!(score, 12);
}
