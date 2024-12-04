use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use crate::file_helper::get_file;

pub fn run() {
    println!("Executing day 4!");

    let file = get_file("./inputs/day4.txt").unwrap();

    let reader = io::BufReader::new(file);

    let grid = parse_input(reader);

    let solution_part_one = part_one(&grid);
    println!("The solution for day 4, part a is {}", solution_part_one);
}

fn parse_input(reader: BufReader<File>) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        grid.push(line.unwrap().chars().collect::<Vec<char>>());
    }

    grid
}

fn get_dirs() -> Vec<(isize, isize)> {
    vec![
        (0, 1),   // Horizontal right
        (0, -1),  // Horizontal left
        (1, 0),   // Vertical down
        (-1, 0),  // Vertical up
        (1, 1),   // Diagonal down-right
        (1, -1),  // Diagonal down-left
        (-1, 1),  // Diagonal up-right
        (-1, -1), // Diagonal up-left
    ]
}

fn part_one(grid: &Vec<Vec<char>>) -> i32 {
    let directions = get_dirs();

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
}
