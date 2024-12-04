//use crate::solutions::day04::TransposeDirection::{Backwards, Forwards};

//const XMAS: &'static str = "XMAS";
//const MAS: &'static str = "MAS";
//const SAMX: &'static str = "SAMX";

pub fn solve_day_04(input: &str) -> (usize, i32) {
    let p1 = part1_fast(&input);
    let p2 = part2_fast(input);

    (p1, p2)
}

fn part1_fast(input: &&str) -> usize {
    let char_matrix: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
    let mut result = 0;
    // this breaks for non-square grids
    let height_width = char_matrix.len();
    let chars = b"MAS";

    for x in 0..height_width {
        for y in 0..height_width {

            if char_matrix[y][x] == b'X' {
                let directions: [(isize, isize); 8] = [(-1, -1), (0, -1), (1, -1), (-1, 0),
                    (1, 0), (-1, 1), (0, 1), (1, 1),];

                for (dx, dy) in directions {
                    let mut found = true;

                    for i in 1..=3 {
                        let new_x = x as isize + dx * i;
                        let new_y = y as isize + dy * i;

                        // Check boundaries and match characters
                        if new_x >= 0 && new_x < height_width as isize && new_y >= 0 && new_y < height_width as isize {
                            let ch = char_matrix[new_y as usize][new_x as usize];
                            if ch != chars[(i - 1) as usize] {
                                found = false;
                                break;
                            }
                        } else {
                            found = false;
                            break;
                        }
                    }

                    if found {
                        result += 1;
                    }
                }

            }
        }
    }

    result
}

fn part2_fast(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    // this breaks for non-square grids
    let height_width = grid.len();
    let mut p2 = 0;
    // Iterate over the grid, avoiding the edges
    for idx in 1..height_width - 1 {
        for cidx in 1..height_width - 1 {
            if grid[idx][cidx] == 'A' {
                let xmas = XMAS {
                    top_left: grid[idx - 1][cidx - 1],
                    top_right: grid[idx - 1][cidx + 1],
                    bottom_left: grid[idx + 1][cidx - 1],
                    bottom_right: grid[idx + 1][cidx + 1],
                };
                if xmas.has_valid_pattern() {
                    p2 += 1;
                }
            }
        }
    }
    p2
}

// slower part 2
// fn part2(input: &str) -> i32 {
//     let mut p2 = 0;
//     let all_lines: Vec<&str> = input.lines().collect();
//     for (idx, line) in all_lines.clone().into_iter().enumerate() {
//         for (cidx, c) in line.chars().enumerate() {
//             if c == 'A' && not_at_the_edge(idx, line, cidx) {
//                 let chars = XMAS {
//                     top_left: all_lines[idx - 1].chars().nth(cidx - 1).unwrap(),
//                     bottom_right: all_lines[idx + 1].chars().nth(cidx + 1).unwrap(),
//                     top_right: all_lines[idx - 1].chars().nth(cidx + 1).unwrap(),
//                     bottom_left: all_lines[idx + 1].chars().nth(cidx - 1).unwrap(),
//                 };
//                 if chars.has_valid_pattern() {
//                     p2 += 1;
//                 }
//             }
//         }
//     }
//     p2
// }
//
// fn not_at_the_edge(idx: usize, line: &str, cidx: usize) -> bool {
//     idx > 0 && idx < line.len() - 1 && cidx > 0 && cidx < line.len() - 1
// }

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

// slower part 1
// fn part1(input: &&str) -> usize {
//     input.lines()
//         .chain(transpose_diagonal(input, Forwards).lines())
//         .chain(transpose(input).lines())
//         .chain(transpose_diagonal(input, Backwards).lines())
//         .map(|line| {
//             line.matches(XMAS).count() + line.matches(SAMX).count()
//         })
//         .sum()
// }
//
//
// enum TransposeDirection {
//     Forwards,
//     Backwards,
// }
//
// fn transpose_diagonal(input: &str, direction: TransposeDirection) -> String {
//     let char_matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
//
//     // should be the same for our case.
//     let rows = char_matrix.len();
//     let cols = char_matrix[0].len();
//     let mut new_lines = Vec::new();
//
//     match direction {
//         TransposeDirection::Forwards => {
//             for current_line in 0..(rows + cols - 1) {
//                 let mut diagonal = Vec::new();
//                 for i in 0..rows {
//                     if i > current_line {
//                         continue;
//                     }
//                     let j = current_line - i;
//                     if j < cols {
//                         diagonal.push(char_matrix[i][j]);
//                     }
//                 }
//                 if !diagonal.is_empty() {
//                     new_lines.push(diagonal.into_iter().collect::<String>());
//                 }
//             }
//         }
//         TransposeDirection::Backwards => {
//             for diff in -(cols as i32 - 1)..rows as i32 {
//                 let mut diagonal = Vec::new();
//                 for i in 0..rows {
//                     let j = i as i32 - diff;
//                     if j >= 0 && j < cols as i32 {
//                         diagonal.push(char_matrix[i][j as usize]);
//                     }
//                 }
//                 if !diagonal.is_empty() {
//                     new_lines.push(diagonal.into_iter().collect::<String>());
//                 }
//             }
//         }
//     }
//
//     new_lines.join("\n")
// }
//
// fn transpose(input: &str) -> String {
//     let lines: Vec<&str> = input.lines().collect();
//     let len = lines.len();
//     let mut result = String::new();
//
//     for i in 0..len {
//         for line in lines.iter() {
//             if let Some(ch) = line.chars().nth(i) {
//                 result.push(ch);
//             }
//         }
//         result.push('\n');
//     }
//
//     result.trim_end().to_string()
// }

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
