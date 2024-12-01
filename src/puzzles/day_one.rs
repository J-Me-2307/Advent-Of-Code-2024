use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub fn run_day_one() {
    println!("Executing day 1");

    let path: &Path = Path::new("./inputs/day_one.txt");

    let file: File = File::open(&path).unwrap();
    let reader = io::BufReader::new(file);

    let mut left_numbers: Vec<i32> = Vec::new();
    let mut right_numbers: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        if let Some((left, right)) = line.split_once("   ") {
            if let (Ok(left_num), Ok(right_num)) = (left.parse::<i32>(), right.parse::<i32>()) {
                left_numbers.push(left_num);
                right_numbers.push(right_num);
            }
        }
    }

    left_numbers.sort();
    right_numbers.sort();

    let solution_part_one = part_one(&left_numbers, &right_numbers);
    println!("The solution for day 1, part a is {}", solution_part_one);
    let solution_part_two = part_two(&left_numbers, &right_numbers);
    println!("The solution for day 1, part b is {}", solution_part_two);
}

fn part_one(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    let mut difference = 0;

    for i in 0..left.len() {
        difference += (left[i] - right[i]).abs();
    }

    difference
}

fn part_two(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    let mut similarity_score = 0;

    for i in left {
        let mut count = 0;

        for j in right {
            if i == j{
                count += 1;
            }
        }

        similarity_score += count * i;
    }

    similarity_score
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one_test() {
        // Arrange
        let mut left = vec![3, 4, 2, 1, 3, 3];
        left.sort();
        let mut right = vec![4, 3, 5, 3, 9, 3];
        right.sort();

        let expected = 11;

        // Act
        let actual = part_one(&left, &right);

        // Assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn part_two_test() {
        // Arrange
        let mut left = vec![3, 4, 2, 1, 3, 3];
        left.sort();
        let mut right = vec![4, 3, 5, 3, 9, 3];
        right.sort();

        let expected = 31;

        // Act
        let actual = part_two(&left, &right);

        // Assert
        assert_eq!(expected, actual)
    }
}
