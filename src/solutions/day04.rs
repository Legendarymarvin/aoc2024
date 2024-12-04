use crate::solutions::day04::TransposeDirection::{Backwards, Forwards};

const XMAS: &'static str = "XMAS";
const SAMX: &'static str = "SAMX";

pub fn solve_day_04(input: &str) -> (usize, i32) {
    let p1 = part1(&input);
    let p2 = part2(input);

    (p1, p2)
}


fn part1(input: &&str) -> usize {
    input.lines()
        .chain(transpose_diagonal(input, Forwards).lines())
        .chain(transpose(input).lines())
        .chain(transpose_diagonal(input, Backwards).lines())
        .map(|line| {
            line.matches(XMAS).count() + line.matches(SAMX).count()
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    let mut p2 = 0;
    let all_lines: Vec<&str> = input.lines().collect();
    for (idx, line) in all_lines.clone().into_iter().enumerate() {
        for (cidx, c) in line.chars().enumerate() {
            if c == 'A' && not_at_the_edge(idx, line, cidx) {
                let chars = XMAS {
                    top_left: all_lines[idx - 1].chars().nth(cidx - 1).unwrap(),
                    bottom_right: all_lines[idx + 1].chars().nth(cidx + 1).unwrap(),
                    top_right: all_lines[idx - 1].chars().nth(cidx + 1).unwrap(),
                    bottom_left: all_lines[idx + 1].chars().nth(cidx - 1).unwrap(),
                };
                if chars.has_valid_pattern() {
                    p2 += 1;
                }
            }
        }
    }
    p2
}

fn not_at_the_edge(idx: usize, line: &str, cidx: usize) -> bool {
    idx > 0 && idx < line.len() - 1 && cidx > 0 && cidx < line.len() - 1
}

struct XMAS {
    top_left: char,
    top_right: char,
    bottom_left: char,
    bottom_right: char,
}

impl XMAS {
    fn has_valid_pattern(&self) -> bool {
        let chars = [
            self.top_left,
            self.top_right,
            self.bottom_left,
            self.bottom_right,
        ];

        chars.iter().filter(|&&c| c == 'M').count() == 2 &&
            chars.iter().filter(|&&c| c == 'S').count() == 2 &&
            self.top_right != self.bottom_left &&
            self.top_left != self.bottom_right
    }
}

enum TransposeDirection {
    Forwards,
    Backwards,
}

fn transpose_diagonal(input: &str, direction: TransposeDirection) -> String {
    let char_matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // should be the same for our case.
    let rows = char_matrix.len();
    let cols = char_matrix[0].len();
    let mut new_lines = Vec::new();

    match direction {
        TransposeDirection::Forwards => {
            for current_line in 0..(rows + cols - 1) {
                let mut diagonal = Vec::new();
                for i in 0..rows {
                    if i > current_line {
                        continue;
                    }
                    let j = current_line - i;
                    if j < cols {
                        diagonal.push(char_matrix[i][j]);
                    }
                }
                if !diagonal.is_empty() {
                    new_lines.push(diagonal.into_iter().collect::<String>());
                }
            }
        }
        TransposeDirection::Backwards => {
            for diff in -(cols as i32 - 1)..rows as i32 {
                let mut diagonal = Vec::new();
                for i in 0..rows {
                    let j = i as i32 - diff;
                    if j >= 0 && j < cols as i32 {
                        diagonal.push(char_matrix[i][j as usize]);
                    }
                }
                if !diagonal.is_empty() {
                    new_lines.push(diagonal.into_iter().collect::<String>());
                }
            }
        }
    }

    new_lines.join("\n")
}

fn transpose(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let len = lines.len();
    let mut result = String::new();

    for i in 0..len {
        for line in lines.iter() {
            if let Some(ch) = line.chars().nth(i) {
                result.push(ch);
            }
        }
        result.push('\n');
    }

    result.trim_end().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let test_input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";


        let (p1, p2) = solve_day_04(test_input);

        assert_eq!(p1, 18, "Part 1 failed");
        assert_eq!(p2, 9, "Part 2 failed");
    }
    #[test]
    fn test_solve_real() {
        let test_input = include_str!("../bin/inputs/input04.txt");

        let (p1, p2) = solve_day_04(test_input);

        assert_eq!(p1, 2493, "Part 1 failed");
        assert_eq!(p2, 1890, "Part 2 failed");
    }
}
