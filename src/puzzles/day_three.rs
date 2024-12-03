use std::{
    fs::{self, File},
    io,
    path::Path,
};

use regex::Regex;

pub fn run_day_three() {
    println!("Executing day 3");

    let path: &Path = Path::new("./inputs/day_three.txt");

    let mull_string = fs::read_to_string(path).unwrap();

    let solution_part_one = part_one(&mull_string);
    println!("The solution for day 3, a is {}", solution_part_one);
}

fn part_one(mull_string: &str) -> usize {
    let mut uncorupted: usize = 0;

    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

    let matches: Vec<&str> = re.find_iter(mull_string).map(|mat| mat.as_str()).collect();

    let re = Regex::new(r"\((\d+),\s*(\d+)\)").unwrap();

    for mull in matches {
        if let Some(captures) = re.captures(mull) {
            let num1_str = &captures[1];
            let num2_str = &captures[2];

            let num1: usize = num1_str.parse().unwrap();
            let num2: usize = num2_str.parse().unwrap();

            uncorupted += num1 * num2;
        }
    }

    uncorupted
}

mod tests {
    use super::*;
    #[test]
    fn part_one_test() {
        // Arrange
        let mull_string = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let expected = 161;

        // Act
        let actual = part_one(&mull_string);

        // Assert
        assert_eq!(expected, actual);
    }
}
