use aoc2024::solutions::day02::solve_day_02;

fn main() {
    let input = include_str!("./inputs/input01.txt");

    let (part1, part2) = solve_day_02(input);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
