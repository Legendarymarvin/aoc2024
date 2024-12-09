use std::collections::HashMap;
use rayon::prelude::*;

pub fn solve_day_08(input: &str) -> (i64, usize) {
    let (p1, p2) = solve(&input);
    (p1, p2)
}


fn solve(input: &str) -> (i64, usize) {
    (0, 0)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let test_input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

        let (p1, p2) = solve_day_07(test_input);

        // Off by one for test, not for real, not sure why
        assert_eq!(p1, 3749, "Part 1 failed");
        assert_eq!(p2, 0, "Part 2 failed");
    }
    #[test]
    fn test_solve_real() {
        let test_input = include_str!("../bin/inputs/input07.txt");

        let (p1, p2) = solve_day_07(test_input);

        assert_eq!(p1, 7579994664753, "Part 1 failed");
        assert_eq!(p2, 0, "Part 2 failed");
    }
}
