pub fn solve_day_02(input: &str) -> (i32, i32) {
    input.lines()
        .map(|line| {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            let is_valid_without_removing =
                (is_sorted_asc(&numbers) || is_sorted_desc(&numbers))
                    && adjacent_enough(&numbers);

            if is_valid_without_removing {
                // if it's valid for part 1 it's valid for part 2
                (1, 1)
            } else {
                (0, works_without_one_element(&numbers) as i32)
            }
        })
        .fold((0, 0), |acc, (p1, p2)| (acc.0 + p1, acc.1 + p2))
}

fn works_without_one_element(numbers: &Vec<i32>) -> bool {
    (0..numbers.len()).any(|i| {
        let without: Vec<i32> = numbers
            .iter()
            .enumerate()
            .filter(|&(idx, _)| idx != i)
            .map(|(_, &x)| x)
            .collect();

        adjacent_enough(&without) && (is_sorted_asc(&without) || is_sorted_desc(&without))
    })
}

fn adjacent_enough(vec: &[i32]) -> bool {
    vec.windows(2).all(|w| {
        // 1 <= (w[0] - w[1]).abs() && (w[0] - w[1]).abs() <= 3
        // from 387 to 335 median
        let diff = (w[0] - w[1]).abs();
        diff >= 1 && diff <= 3
    })
}

fn is_sorted_asc(vec: &[i32]) -> bool {
    vec.windows(2).all(|w| w[0] <= w[1])
}

fn is_sorted_desc(vec: &[i32]) -> bool {
    vec.windows(2).all(|w| w[0] >= w[1])
}

// just for benching the performance difference, less than I thought median 432 vs 369
// fn is_sorted_asc(vec: &[i32]) -> bool {
//     let mut sorted = vec.to_vec();
//     sorted.sort();
//     vec == sorted}
//
// fn is_sorted_desc(vec: &[i32]) -> bool {
//     let mut sorted = vec.to_vec();
//     sorted.sort_by(|a, b| b.cmp(a));  // Sort descending directly
//     vec == sorted
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let test_input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        let (sum, similarity) = solve_day_02(test_input);

        // Test Part 1
        let expected_sum = 2;
        assert_eq!(sum, expected_sum, "Part 1 failed");

        // Test Part 2
        let expected_similarity = 4;
        assert_eq!(similarity, expected_similarity, "Part 2 failed");
    }
    #[test]
    fn test_solve_real() {
        let test_input = include_str!("../bin/inputs/input02.txt");

        let (sum, similarity) = solve_day_02(test_input);

        // Test Part 1
        let expected_sum = 442;
        assert_eq!(sum, expected_sum, "Part 1 failed");

        // Test Part 2
        let expected_similarity = 493;
        assert_eq!(similarity, expected_similarity, "Part 2 failed");
    }
}

// tried to use bigger windows and handle cases, but doesn't seem to want to work
//     fn adjacent_enough_2(vec: &Vec<i32>) -> bool {
//     if adjacent_enough(&vec[1..]) {
//         return true;
//     }
//     if adjacent_enough(&vec[..vec.len() - 1]) {
//         return true;
//     }
//     let mut happened = 0;
//     vec.windows(3).all(|w| {
//         if happened > 1 {
//             return false;
//         }
//         if !(1 <= (w[0] - w[1]).abs() && (w[0] - w[1]).abs() <= 3) && (1 <= (w[0] - w[2]).abs() && (w[0] - w[2]).abs() <= 3) {
//             happened += 1;
//         }
//         (1 <= (w[0] - w[1]).abs() && (w[0] - w[1]).abs() <= 3)
//         || (1 <= (w[0] - w[2]).abs() && (w[0] - w[2]).abs() <= 3)
//     })
// }
//
// fn is_sorted_asc_2(vec: &[i32]) -> bool {
//     let mut happened = 0;
//     if is_sorted_asc(&vec[1..]) {
//         return true;
//     }
//     if is_sorted_asc(&vec[..vec.len() - 1]) {
//         return true;
//     }
//     vec.windows(3).all(|w| {
//         if (happened > 1) {
//             return false;
//         }
//         if !(w[0] <= w[1]) && (w[0] <= w[2]) {
//             happened += 1;
//         }
//         w[0] <= w[1] || w[0] <= w[2]
//     })
// }
//
// fn is_sorted_desc_2(vec: &[i32]) -> bool {
//     if is_sorted_desc(&vec[1..]) {
//         return true;
//     }
//     if is_sorted_desc(&vec[..vec.len() - 1]) {
//         return true;
//     }
//     let mut happened = 0;
//     vec.windows(3).all(|w| {
//         if (happened > 1) {
//             return false;
//         }
//         if !(w[0] >= w[1]) && (w[0] >= w[2]) {
//             happened += 1;
//         }
//         w[0] >= w[1] || w[0] >= w[2]
//     })
// }