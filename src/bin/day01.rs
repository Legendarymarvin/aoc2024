use std::collections::HashMap;

fn main() {
    // Benchmark 1: ./target/release/day01
    // Time (mean ± σ):     788.2 µs ±  97.2 µs    [User: 601.3 µs, System: 110.0 µs]
    // Range (min … max):   671.9 µs … 1861.1 µs    3175 runs

    let input = include_str!("./inputs/input01.txt");
    let (sum, similarity) = solve(input);

    println!("Part 1: {}", sum);
    println!("Part 2: {}", similarity);
}

pub fn solve(input: &str) -> (i32, i32) {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            let mut nums = line
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .unzip();

    left.sort();
    right.sort();

    let sum = part1(&mut left, &mut right);
    let similarity = part2(&mut left, &mut right);
    (sum, similarity)
}

fn part1(left: &mut Vec<i32>, right: &mut Vec<i32>) -> i32 {
    let mut sum = 0;
    for (i, l) in left.iter().enumerate() {
        sum += (l - right[i]).abs();
    }
    sum
}

fn part2(left: &mut Vec<i32>, right: &mut Vec<i32>) -> i32 {
    // probably performance improvement for part 2, could also be done in the loop with part 1.
    let right_freq: HashMap<i32, i32> = right
        .iter()
        .fold(HashMap::new(), |mut map, &num| {
            *map.entry(num).or_insert(0) += 1;
            map
        });

    let similarity: i32 = left.iter()
        .map(|&l| l * right_freq.get(&l).unwrap_or(&0))
        .sum();
    similarity
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let test_input = "3   4
4   3
2   5
1   3
3   9
3   3";

        let (sum, similarity) = solve(test_input);

        // Test Part 1
        let expected_sum = 11;
        assert_eq!(sum, expected_sum, "Part 1 failed");

        // Test Part 2
        let expected_similarity = 31;
        assert_eq!(similarity, expected_similarity, "Part 2 failed");
    }
}