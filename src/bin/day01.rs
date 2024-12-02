use aoc2024::solutions::day01::solve_day_01;

fn main() {
    let input = include_str!("./inputs/input01.txt");

    // Benchmark 1: ./target/release/day01
    // Time (mean ± σ):     788.2 µs ±  97.2 µs    [User: 601.3 µs, System: 110.0 µs]
    // Range (min … max):   671.9 µs … 1861.1 µs    3175 runs

    // with sort unstable:
    // Benchmark 1: ./target/release/day01
    // Time (mean ± σ):     775.2 µs ±  87.7 µs    [User: 590.6 µs, System: 113.5 µs]
    // Range (min … max):   659.9 µs … 1790.8 µs    4020 runs

    let (sum, similarity) = solve_day_01(input);

    //  Benchmark 1: ./target/release/day01
    //  Time (mean ± σ):     808.4 µs ±  93.0 µs    [User: 623.4 µs, System: 114.3 µs]
    //  Range (min … max):   672.5 µs … 1567.1 µs    2583 runs
    // let (sum, similarity) = solve2(input);

    println!("Part 1: {}", sum);
    println!("Part 2: {}", similarity);
}
