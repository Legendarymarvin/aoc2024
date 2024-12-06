use aoc2024::solutions::day01::solve_day_01;
use aoc2024::solutions::day02::solve_day_02;
use aoc2024::solutions::day03::solve_day_03;
use aoc2024::solutions::day04::solve_day_04;
use aoc2024::solutions::day05::solve_day_05;
use aoc2024::solutions::day06::solve_day_06;

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

#[divan::bench(sample_size = 100)]
fn day03() -> (i32, i32) {
    // Timer precision: 20 ns
    // solutions  fastest       │ slowest       │ median        │ mean          │ samples │ iters
    // ╰─ day03   851.9 µs      │ 964.3 µs      │ 872.4 µs      │ 883 µs        │ 100     │ 10000

    let input = include_str!("../src/bin/inputs/input03.txt");
    solve_day_03(input)
}

#[divan::bench(sample_size = 100)]
fn day04() -> (usize, i32) {
    // Timer precision: 20 ns
    // solutions  fastest       │ slowest       │ median        │ mean          │ samples │ iters
    // ╰─ day04   1.924 ms      │ 2.131 ms      │ 2 ms          │ 2.003 ms      │ 100     │ 10000
    // fast versions
    // ╰─ day04   316.6 µs      │ 379.6 µs      │ 353.1 µs      │ 348.4 µs      │ 100     │ 10000

    let input = include_str!("../src/bin/inputs/input04.txt");
    solve_day_04(input)
}

#[divan::bench(sample_size = 100)]
fn day05() -> (i32, i32) {
    // solutions  fastest       │ slowest       │ median        │ mean          │ samples │ iters
    // ╰─ day05   687.7 µs      │ 771.7 µs      │ 711.4 µs      │ 715.8 µs      │ 100     │ 10000

    let input = include_str!("../src/bin/inputs/input05.txt");
    solve_day_05(input)
}

#[divan::bench(sample_size = 1)]
fn day06() -> (usize, usize) {
    // solutions  fastest       │ slowest       │ median        │ mean          │ samples │ iters
    // ╰─ day06   591.6 ms      │ 641.4 ms      │ 599.4 ms      │ 602.3 ms      │ 100     │ 100

    let input = include_str!("../src/bin/inputs/input06.txt");
    solve_day_06(input)
}
