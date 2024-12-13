use std::str::Lines;
use rayon::prelude::*;

pub fn solve_day_13(input: &str) -> (usize, i64) {
    let (p1, p2) = solve(&input);
    (p1, p2)
}

#[derive(Debug)]
struct Machine {
    a_vector: (i64, i64),
    b_vector: (i64, i64),
    target: (i64, i64),
    target_2: (i64, i64),
}
fn get_substring_after_last_equals(s: &str) -> i64 {
    if let Some(pos) = s.rfind('=') {
        s[pos + 1..].parse::<i64>().unwrap()
    } else {
        panic!("what happened here {}", s)
    }
}

fn solve(input: &&str) -> (usize, i64) {
    let machines: Vec<_> = input.split("\n\n").into_iter().map(|machine| {
        let mut lines = machine.lines();
        let a = parse_button_vector(lines.next().unwrap());
        let b = parse_button_vector(lines.next().unwrap());
        let p = parse_price(lines);
        Machine {
            a_vector: a,
            b_vector: b,
            target: p,
            target_2: (10000000000000 + p.0, 10000000000000 + p.1)
        }
    }).collect();
    let p1: i64 = machines.par_iter().map(|machine| {
        machine.can_reach_p1().map(|c| calculate_token_cost(c)).unwrap_or(0)
    }).sum();
    let p2: i64 = machines.par_iter().map(|machine| {
        machine.can_reach_p2().map(|c| calculate_token_cost(c)).unwrap_or(0)
    }).sum();

    (p1 as usize,p2)
}

fn parse_price(mut lines: Lines) -> (i64, i64) {
    let p = lines.next().unwrap().split_once(", ").map(|(l, r)| {
        (get_substring_after_last_equals(l), get_substring_after_last_equals(r))
    }).unwrap();
    p
}

fn parse_button_vector(line: &str) -> (i64, i64) {
    let a = line.split_once(", ").map(|(l, r)| {
        (l.get(12..14).unwrap().parse::<i64>().unwrap(), r.get(2..4).unwrap().parse::<i64>().unwrap())
    }).unwrap();
    a
}

fn calculate_token_cost(c: (i64, i64)) -> i64 {
    c.0 * 3 + c.1
}

// slow bruteforce
#[allow(unused)]
fn can_reach_target(target: (i64, i64), a: (i64, i64), b: (i64, i64)) -> (bool, Vec<(i64, i64)>) {
    let mut possible_combinations = Vec::new();
    let max_steps = (target.0.abs() + target.1.abs()).max(1);

    for i in 0..=max_steps {
        for j in 0..=max_steps {
            let x = i * a.0 as i64 + j * b.0 as i64;
            let y = i * a.1 as i64 + j * b.1 as i64;

            if x == target.0 && y == target.1 {
                possible_combinations.push((i, j));
            }
        }
    }

    (possible_combinations.len() > 0, possible_combinations)
}

impl Machine {
    fn can_reach_p1(&self) -> Option<(i64, i64)> {
        can_reach_target_fast(self.target, self.a_vector, self.b_vector)
    }

    fn can_reach_p2(&self) -> Option<(i64, i64)> {
        can_reach_target_fast(self.target_2, self.a_vector, self.b_vector)
    }
}
fn can_reach_target_fast(target: (i64, i64), a_vector: (i64, i64), b_vector: (i64, i64)) -> Option<(i64, i64)> {
    // need to find a value x and a value y that resolves to
    // A.0 * x and B.0 * y = P.0 and
    // A.1 * x and B.1 * y = P.1
    let determinant = calculate_determinant(a_vector, b_vector);

    // Cramer's Rule
    let x = target.0 * b_vector.1 - target.1 * b_vector.0;
    let y = a_vector.0 * target.1 - a_vector.1 * target.0;

    // looking for determinant not 0 and that divides x & y evenly (can't press buttons partially)
    if determinant == 0 || x % determinant != 0 || y % determinant != 0 {
        return None;
    }

    Some((x / determinant, y / determinant))
}

fn calculate_determinant(a: (i64, i64), b: (i64, i64)) -> i64 {
    a.0 * b.1 - a.1 * b.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let test_input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

        let (p1, _p2) = solve_day_13(test_input);

        assert_eq!(p1, 480, "Part 1 failed");

//         let test_input = "Button A: X+94, Y+34
// Button B: X+22, Y+67
// Prize: X=10000000008400, Y=10000000005400
//
// Button A: X+26, Y+66
// Button B: X+67, Y+21
// Prize: X=10000000012748, Y=10000000012176
//
// Button A: X+17, Y+86
// Button B: X+84, Y+37
// Prize: X=10000000007870, Y=10000000006450
//
// Button A: X+69, Y+23
// Button B: X+27, Y+71
// Prize: X=10000000018641, Y=10000000010279";
//         let (_p1, p2) = solve_day_13(test_input);
//         assert_eq!(p2, 875318608908, "Part 2 failed");
    }
    #[test]
    fn test_solve_real() {
        let test_input = include_str!("../bin/inputs/input13.txt");

        let (p1, p2) = solve_day_13(test_input);

        assert_eq!(p1, 29598, "Part 1 failed");
        assert_eq!(p2, 93217456941970, "Part 2 failed");
    }
}
