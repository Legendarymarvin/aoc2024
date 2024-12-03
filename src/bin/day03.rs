use aoc2024::solutions::day03::solve_day_03;

fn main() {
    let input = include_str!("./inputs/input03.txt");

    let (part1, part2) = solve_day_03(input);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
