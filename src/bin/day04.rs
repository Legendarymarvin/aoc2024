use aoc2024::solutions::day04::solve_day_04;

fn main() {
    let input = include_str!("./inputs/input04.txt");

    let (part1, part2) = solve_day_04(input);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
