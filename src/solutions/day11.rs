use std::collections::HashMap;

const BLINKS_P1: usize = 25;
const BLINKS_P2: usize = 75;
const MUL: u64 = 2024;

pub fn solve_day_11(input: &str) -> (usize, usize) {
    let (p1, p2) = solve(&input);
    (p1, p2)
}

fn solve(input: &str) -> (usize, usize) {
    let mut stones = HashMap::new();

    // apparently stones are unique in the input, so this is unnecessarily complicated.
    input.lines().next().expect("empty input").split_whitespace().for_each(|stone| {
        *stones.entry(stone.parse::<u64>().expect("invalid stone")).or_insert(0) += 1;
    });

    // doing part 1 separately instead of combining with part 2 costs only half a ms, but would be an easy fix.
    let p1 = blink_x_times(&stones, BLINKS_P1);
    let p2 = blink_x_times(&stones, BLINKS_P2);
    (p1, p2)
}

fn blink_x_times(initial_stones: &HashMap<u64, usize>, blinks: usize) -> usize {
    let final_stones = (0..blinks).fold(initial_stones.clone(), |stones, _| blink(&stones));
    final_stones.values().sum()
}


fn blink(old_stones: &HashMap<u64, usize>) -> HashMap<u64, usize> {
    let mut moving_stones = HashMap::new();
    for (&stone, &value) in old_stones {
        if stone == 0 {
            *moving_stones.entry(1).or_default() += value
        } else {
            let length = count_digits(stone);
            match length % 2 == 0 {
                true => {
                    let (left, right) = split_stone(stone, length);
                    *moving_stones.entry(left).or_default() += value;
                    *moving_stones.entry(right).or_default() += value;
                },
                false => {
                    *moving_stones.entry(stone * MUL).or_default() += value
                }
            }
        }
    }
    moving_stones
}

fn count_digits(mut n: u64) -> u32 {
    if n == 0 {
        return 1;
    }
    let mut digits = 0;
    while n > 0 {
        digits += 1;
        n /= 10;
    }
    digits
}


fn split_stone(n: u64, digits: u32) -> (u64, u64) {
    let half = digits / 2;
    let divisor = 10u64.pow(half);
    let left = n / divisor;
    let right = n % divisor;
    (left, right)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let test_input = "125 17";

        let (p1, p2) = solve_day_11(test_input);

        assert_eq!(p1, 55312, "Part 1 failed");
        assert_eq!(p2, 65601038650482, "Part 2 failed");
    }
    #[test]
    fn test_solve_real() {
        let test_input = include_str!("../bin/inputs/input11.txt");

        let (p1, p2) = solve_day_11(test_input);

        assert_eq!(p1, 183248, "Part 1 failed");
        assert_eq!(p2, 218811774248729, "Part 2 failed");
    }
}
