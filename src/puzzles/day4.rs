use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    isize,
};

use crate::file_helper::get_file;

pub fn run() {
    println!("Executing day 4!");

    let file = get_file("./inputs/day4.txt").unwrap();

    let reader = io::BufReader::new(file);

    let grid = parse_input(reader);

    let solution_part_one = part_one(&grid);
    println!("The solution for day 4, part a is {}", solution_part_one);

    let solution_part_two = part_two(&grid);
    println!("The solution for day 4, part b is {}", solution_part_two);
}

fn parse_input(reader: BufReader<File>) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        grid.push(line.unwrap().chars().collect::<Vec<char>>());
    }

    grid
}

fn part_one(grid: &Vec<Vec<char>>) -> i32 {
    let directions = vec![
        (0, 1),   // Horizontal right
        (0, -1),  // Horizontal left
        (1, 0),   // Vertical down
        (-1, 0),  // Vertical up
        (1, 1),   // Diagonal down-right
        (1, -1),  // Diagonal down-left
        (-1, 1),  // Diagonal up-right
        (-1, -1), // Diagonal up-left
    ];

    let mut count = 0;
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            for dir in &directions {
                if search(&grid, "XMAS", (row, col), *dir) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn search(
    grid: &Vec<Vec<char>>,
    word: &str,
    start: (usize, usize),
    direction: (isize, isize),
) -> bool {
    let (mut x, mut y) = (start.0 as isize, start.1 as isize);

    for c in word.chars() {
        if x < 0 || y < 0 || x >= grid.len() as isize || y >= grid[0].len() as isize {
            return false;
        }
        if grid[x as usize][y as usize] != c {
            return false;
        }
        x += direction.0;
        y += direction.1;
    }
    true
}

fn part_two(grid: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    let rows = grid.len();
    let cols = grid[0].len();
    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            if row >= 1 && row < grid.len() - 1 && col >= 1 && col < grid[row].len() - 1 {
                if grid[row][col] == 'A' && is_x(&grid, (row, col)) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn is_x(grid: &[Vec<char>], (row, col): (usize, usize)) -> bool {
    let top_left = grid[row - 1][col - 1];
    let top_right = grid[row - 1][col + 1];
    let bottom_left = grid[row + 1][col - 1];
    let bottom_right = grid[row + 1][col + 1];

    // Check if corners are valid
    if !matches!(top_left, 'M' | 'S')
        || !matches!(top_right, 'M' | 'S')
        || !matches!(bottom_left, 'M' | 'S')
        || !matches!(bottom_right, 'M' | 'S')
    {
        return false;
    }

    let diagonal1 = format!("{}{}{}", top_left, grid[row][col], bottom_right);
    let diagonal2 = format!("{}{}{}", top_right, grid[row][col], bottom_left);

    (diagonal1 == "SAM" || diagonal1 == "MAS") && (diagonal2 == "SAM" || diagonal2 == "MAS")
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part_one_test() {
        // Arrange
        let input: Vec<Vec<char>> = vec![
            "MMMSXXMASM".chars().collect::<Vec<char>>(),
            "MSAMXMSMSA".chars().collect::<Vec<char>>(),
            "AMXSXMAAMM".chars().collect::<Vec<char>>(),
            "MSAMASMSMX".chars().collect::<Vec<char>>(),
            "XMASAMXAMM".chars().collect::<Vec<char>>(),
            "XXAMMXXAMA".chars().collect::<Vec<char>>(),
            "SMSMSASXSS".chars().collect::<Vec<char>>(),
            "SAXAMASAAA".chars().collect::<Vec<char>>(),
            "MAMMMXMMMM".chars().collect::<Vec<char>>(),
            "MXMXAXMASX".chars().collect::<Vec<char>>(),
        ];

        let expected = 18;

        // Act
        let actual = part_one(&input);

        // Assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn part_two_test() {
        let input: Vec<Vec<char>> = vec![
            "MMMSXXMASM".chars().collect::<Vec<char>>(),
            "MSAMXMSMSA".chars().collect::<Vec<char>>(),
            "AMXSXMAAMM".chars().collect::<Vec<char>>(),
            "MSAMASMSMX".chars().collect::<Vec<char>>(),
            "XMASAMXAMM".chars().collect::<Vec<char>>(),
            "XXAMMXXAMA".chars().collect::<Vec<char>>(),
            "SMSMSASXSS".chars().collect::<Vec<char>>(),
            "SAXAMASAAA".chars().collect::<Vec<char>>(),
            "MAMMMXMMMM".chars().collect::<Vec<char>>(),
            "MXMXAXMASX".chars().collect::<Vec<char>>(),
        ];
        let expected = 9;

        // Act
        let actual = part_two(&input);

        // Assert
        assert_eq!(expected, actual);
    }
}
