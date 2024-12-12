use aoc2024::solutions::day01::solve_day_01;
use aoc2024::solutions::day02::solve_day_02;
use aoc2024::solutions::day03::solve_day_03;
use aoc2024::solutions::day04::solve_day_04;
use aoc2024::solutions::day05::solve_day_05;
use aoc2024::solutions::day06::solve_day_06;
use aoc2024::solutions::day07::solve_day_07;
use aoc2024::solutions::day09::solve_day_09;
use aoc2024::solutions::day10::solve_day_10;
use aoc2024::solutions::day11::solve_day_11;
use aoc2024::solutions::day12::solve_day_12;

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
    // ╰─ day06   143.2 ms      │ 156.4 ms      │ 145.5 ms      │ 146.5 ms      │ 100     │ 100

    let input = include_str!("../src/bin/inputs/input06.txt");
    solve_day_06(input)
}

#[divan::bench(sample_size = 1)]
fn day07() -> (i64, usize) {
    // solutions  fastest       │ slowest       │ median        │ mean          │ samples │ iters
    // ╰─ day07   16.8 ms       │ 25.19 ms      │ 18.05 ms      │ 18.24 ms      │ 100     │ 100
    // only part 1, so kinda pointless.

    let input = include_str!("../src/bin/inputs/input07.txt");
    solve_day_07(input)
}

#[divan::bench(sample_size = 1)]
fn day09() -> (usize, usize) {
    // solutions  fastest       │ slowest       │ median        │ mean          │ samples │ iters
    // ╰─ day09   502 ms        │ 568 ms        │ 522.5 ms      │ 524.9 ms      │ 100     │ 100

    let input = include_str!("../src/bin/inputs/input09.txt");
    solve_day_09(input)
}

#[divan::bench(sample_size = 1)]
fn day10() -> (usize, usize) {
    // solutions  fastest       │ slowest       │ median        │ mean          │ samples │ iters
    // ╰─ day10   1.081 ms      │ 1.383 ms      │ 1.095 ms      │ 1.118 ms      │ 100     │ 100

    let input = include_str!("../src/bin/inputs/input10.txt");
    solve_day_10(input)
}

#[divan::bench(sample_size = 1)]
fn day11() -> (usize, usize) {
    // solutions  fastest       │ slowest       │ median        │ mean          │ samples │ iters
    // ╰─ day11   9.969 ms      │ 12.48 ms      │ 10.15 ms      │ 10.25 ms      │ 100     │ 100

    let input = include_str!("../src/bin/inputs/input11.txt");
    solve_day_11(input)
}

#[divan::bench(sample_size = 1)]
fn day12() -> (usize, usize) {
    // solutions  fastest       │ slowest       │ median        │ mean          │ samples │ iters
    // ╰─ day12   12.21 ms      │ 13.65 ms      │ 13.28 ms      │ 13.19 ms      │ 100     │ 100

    let input = include_str!("../src/bin/inputs/input12.txt");
    solve_day_12(input)
}
