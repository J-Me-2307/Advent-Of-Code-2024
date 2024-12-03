use std::{
    fs::{self},
    path::Path,
};

use regex::{Matches, Regex};

pub fn run_day_three() {
    println!("Executing day 3");

    let path: &Path = Path::new("./inputs/day_three.txt");

    let mul_string = fs::read_to_string(path).unwrap();

    let solution_part_one = part_one(&mul_string);
    println!("The solution for day 3, a is {}", solution_part_one);

    let solution_part_two = part_two(&mul_string);
    println!("The solution for day 3, b is {}", solution_part_two);
}

fn part_one(mul_string: &str) -> usize {
    let mut uncorupted: usize = 0;

    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

    let matches: Vec<&str> = re.find_iter(mul_string).map(|mat| mat.as_str()).collect();

    let re = Regex::new(r"\((\d+),\s*(\d+)\)").unwrap();

    for mul in matches {
        if let Some(captures) = re.captures(mul) {
            let num1_str = &captures[1];
            let num2_str = &captures[2];

            let num1: usize = num1_str.parse().unwrap();
            let num2: usize = num2_str.parse().unwrap();

            uncorupted += num1 * num2;
        }
    }

    uncorupted
}

fn part_two(mul_string: &str) -> usize {
    let mut uncorupted: usize = 0;

    let re = Regex::new(r"(mul\((\d{1,3}),\s*(\d{1,3})\)|do\(\)|don't\(\))").unwrap();

    let matches: Vec<&str> = re.find_iter(mul_string).map(|mat| mat.as_str()).collect();

    let re = Regex::new(r"\((\d+),\s*(\d+)\)").unwrap();

    let mut do_multiply = true;

    for mat in matches {
        match mat {
            "don't()" => do_multiply = false,
            "do()" => do_multiply = true,
            _ => {
                if do_multiply == true {
                    if let Some(captures) = re.captures(mat) {
                        let num1_str = &captures[1];
                        let num2_str = &captures[2];

                        let num1: usize = num1_str.parse().unwrap();
                        let num2: usize = num2_str.parse().unwrap();

                        uncorupted += num1 * num2;
                    }
                }
            }
        }
    }

    uncorupted
}

mod tests {
    #[test]
    fn part_one_test() {
        // Arrange
        let mul_string = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let expected = 161;

        // Act
        let actual = crate::puzzles::day_three::part_one(&mul_string);

        // Assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn part_two_test() {
        // Arrange
        let mul_string =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let expected = 48;

        // Act
        let actual = crate::puzzles::day_three::part_two(&mul_string);

        // Assert
        assert_eq!(expected, actual);
    }
}
