use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub fn run_day_two() {
    println!("Executing day 2");

    let path: &Path = Path::new("./inputs/day_two.txt");

    let file = File::open(&path).expect("Failed to open the file");
    let reader = io::BufReader::new(file);

    let mut lines_list = Vec::new();

    for line in reader.lines() {
        let mut line_list = Vec::new();
        match line {
            Ok(content) => {
                for word in content.split_whitespace() {
                    if let Ok(number) = word.parse::<i32>() {
                        line_list.push(number);
                    }
                }
            }
            Err(err) => {
                eprintln!("Error reading a line: {}", err);
            }
        }
        lines_list.push(line_list);
    }

    let solution_part_one = part_one(&lines_list);
    println!("The solution for day 2, part a is {}", solution_part_one);

    let solution_part_two = part_two(&lines_list);
    println!("The solution for day 2, part b is {}", solution_part_two);
}

fn part_one(lines: &Vec<Vec<i32>>) -> i32 {
    let mut safe_reports = 0;

    for line in lines {
        if is_safe(&line) {
            safe_reports += 1;
        }
    }

    safe_reports
}

fn part_two(lines: &Vec<Vec<i32>>) -> i32 {
    let mut safe_reports = 0;

    for line in lines {
        if is_safe(&line) {
            safe_reports += 1;
        } else {
            for i in 0..line.len(){
                let mut line_without_i = Vec::new();
                line_without_i.extend_from_slice(&line[0..i]);
                line_without_i.extend_from_slice(&line[i+1..]);

                if is_safe(&line_without_i){
                    safe_reports += 1;
                    break;
                }
            }
        }
    }

    safe_reports
}

fn is_safe(line: &Vec<i32>) -> bool {
    let is_consistent = line.windows(2).all(|pair| pair[0] < pair[1])
        || line.windows(2).all(|pair| pair[0] > pair[1]);
    let difference_smaller_four = line.windows(2).all(|pair| (pair[0] - pair[1]).abs() <= 3);

    is_consistent && difference_smaller_four
}

mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        // Arrange
        let lines = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];

        let expected = 2;

        // Act
        let actual = part_one(&lines);

        // Assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn part_two_test() {
        // Arrange
        let lines = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];

        let expected = 4;

        // Act
        let actual = part_two(&lines);

        // Assert
        assert_eq!(expected, actual);
    }
}
