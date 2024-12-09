use aoc2024::solutions::day08::solve_day_08;

fn main() {
    let input = include_str!("./inputs/input08t.txt");

    let (part1, part2) = solve_day_08(input);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
