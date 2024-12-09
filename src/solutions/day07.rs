use rayon::prelude::*;

pub fn solve_day_07(input: &str) -> (i64, usize) {
    let (p1, p2) = solve(&input);
    (p1, p2)
}

#[derive(Clone, Debug)]
struct Equation {
    result: i64,
    operands: Vec<i64>,
}

fn solve(input: &str) -> (i64, usize) {
    let equations = input.lines().map(|line| {
        let (result, ops) = line.split_once(": ").unwrap();
        Equation {
            result: result.parse::<i64>().unwrap(),
            operands: ops.split(' ').map(|o| o.parse::<i64>().unwrap()).collect()
        }
    }).collect::<Vec<_>>();
    let res = equations.clone().par_iter()
        .filter_map(|equation| {
            find_expression(&equation)
        })
        .sum::<i64>();

    (res, 0)
}
fn find_expression(
    equ: &Equation
) -> Option<i64> {
    let perms = generate_expressions(&equ.operands);
    for perm in perms.iter(){
        let pot_res = evaluate(perm);
        if pot_res == equ.result {
            return Some(pot_res);
        }
    };
    None
}

fn evaluate(expr: &String) -> i64 {
    let tokens = tokenize(expr);
    let result = evaluate_tokens(&tokens);
    result
}

fn evaluate_tokens(tokens: &[String]) -> i64 {
    let mut iter = tokens.iter();
    let mut result = iter
        .next()
        .unwrap()
        .parse::<i64>()
        .unwrap();

    while let Some(op) = iter.next() {
        let next_num = iter
            .next()
            .expect("should be a number")
            .parse::<i64>()
            .expect("should be a number");

        match op.as_str() {
            "+" => result += next_num,
            "*" => result *= next_num,
            _ => panic!("What is this '{}'", op),
        }
    }

    result
}

fn tokenize(expr: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut num = String::new();

    for c in expr.chars() {
        if c.is_digit(10) {
            num.push(c);
        } else if c == '+' || c == '*'  {
            if !num.is_empty() {
                tokens.push(num.clone());
                num.clear();
            }
            tokens.push(c.to_string());
        } else if c.is_whitespace() || c == '|'{
            continue;
        } else {
            panic!("What is this '{}'", c);
        }
    }

    if !num.is_empty() {
        tokens.push(num);
    }

    tokens
}

fn generate_expressions(nums: &[i64]) -> Vec<String> {
    if nums.is_empty() {
        return Vec::new();
    }
    let mut expressions = Vec::new();
    let initial_expr = nums[0].to_string();

    generate_expressions_rec(nums, 1, initial_expr, &mut expressions);

    expressions
}

fn generate_expressions_rec(
    nums: &[i64],
    index: usize,
    current_expr: String,
    expressions: &mut Vec<String>,
) {
    if index == nums.len() {
        expressions.push(current_expr);
        return;
    }

    let expr_plus = format!("{}+{}", current_expr, nums[index]);
    generate_expressions_rec(nums, index + 1, expr_plus, expressions);

    let expr_mul = format!("{}*{}", current_expr, nums[index]);
    generate_expressions_rec(nums, index + 1, expr_mul, expressions);
    // doesn't work for part 2 since it doesn't evaluate before concetanating
    // hence 6 * 8 || 6 * 15 will not be 6*8 = 48 || 6 => 486
    // just 6 * 86 * 15
    // let expr_con = format!("{}|{}", current_expr, nums[index]);
    // generate_expressions_rec(nums, index + 1, expr_con, expressions);

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
