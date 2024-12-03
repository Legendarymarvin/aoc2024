use regex::Regex;

const DONT: &'static str = "don't()";
const DO: &'static str = "do()";

pub fn solve_day_03(input: &str) -> (i32, i32) {
    // first group is the mul, can be ignored, [1] and [2] are the numbers.
    let re1 = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let re2 = Regex::new(r"^mul\((\d+),(\d+)\)").unwrap();
    let mut p1 = 0;
    let mut p2 = 0;
    let oneline = input.replace("\n", "");
    let line = oneline.lines().next().unwrap();

    // part 1
    for cap in re1.captures_iter(line) {
        let x = cap[1].parse::<i32>().unwrap();
        let y = cap[2].parse::<i32>().unwrap();
        p1 += x * y;
    }

    // part 2
    let mut idx = 0;
    let mut doit = true;
    while idx < line.len() {
        if line[idx..].starts_with(DONT) {
            doit = false;
            idx += DONT.len();
        } else if line[idx..].starts_with(DO) {
            doit = true;
            idx += DO.len();
        } else if let Some(cap) = re2.captures(&line[idx..]) {
            // if we are currently in a don't block, don't calculate but move the cursor
            idx += cap[0].len();
            if doit {
                let x = cap[1].parse::<i32>().unwrap();
                let y = cap[2].parse::<i32>().unwrap();
                p2 += x * y;
            }
        } else {
            idx += 1;
        }
    };

    (p1, p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let test_input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let test_input2 = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        let (p1, _p2) = solve_day_03(test_input);
        let (_p1, p2) = solve_day_03(test_input2);

        assert_eq!(p1, 161, "Part 1 failed");
        assert_eq!(p2, 48, "Part 2 failed");
    }
    #[test]
    fn test_solve_real() {
        let test_input = include_str!("../bin/inputs/input03.txt");

        let (p1, p2) = solve_day_03(test_input);

        assert_eq!(p1, 161085926, "Part 1 failed");
        assert_eq!(p2, 82045421, "Part 2 failed");
    }
}