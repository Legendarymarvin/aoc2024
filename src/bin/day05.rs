use aoc2024::solutions::day05::solve_day_05;

fn main() {
    let input = include_str!("./inputs/input05t.txt");

    let (part1, part2) = solve_day_05(input);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
