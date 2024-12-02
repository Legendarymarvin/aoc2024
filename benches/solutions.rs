use aoc2024::solutions::day01::solve_day_01;
use aoc2024::solutions::day02::solve_day_02;

fn main() {
    divan::main();
}

#[divan::bench(sample_size = 100)]
fn day01() -> (i32, i32) {
    // Timer precision: 20 ns
    // day01     fastest       │ slowest       │ median        │ mean          │ samples │ iters
    // ╰─ day01  104.1 µs      │ 112.7 µs      │ 105.8 µs      │ 106.2 µs      │ 100     │ 10000
    let input = include_str!("../src/bin/inputs/input01.txt");
    solve_day_01(input)
}

#[divan::bench(sample_size = 100)]
fn day02() -> (i32, i32) {
    // Timer precision: 20 ns
    // solutions  fastest       │ slowest       │ median        │ mean          │ samples │ iters
    // ╰─ day02   353.6 µs      │ 403 µs        │ 378.7 µs      │ 376.4 µs      │ 100     │ 10000

    let input = include_str!("../src/bin/inputs/input02.txt");
    solve_day_02(input)
}
