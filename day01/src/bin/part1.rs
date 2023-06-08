use day01::solve_part1 as solve;

fn main() {
    let file_path = "input/input.dat";

    let max = solve(file_path);

    println!(" How many total Calories is that Elf carrying? {}", max);
}
