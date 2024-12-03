#![allow(unused)]
use std::{
    fs::File,
    io::{self, BufRead, Read},
};

mod corruption;
mod lists;
mod reports;

fn input_data(day: &str, file: &str) -> String {
    format!("inputs/{day}/{file}")
}

fn read_file_contents(path: &str) -> String {
    let mut s = String::new();
    std::fs::File::open(path)
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    s
}

#[cfg(test)]
mod day1 {
    use super::*;
    use lists::*;

    fn test_file(name: &str) -> String {
        read_file_contents(&input_data("day1", name))
    }

    fn parse_lists(content: &str) -> (Vec<i64>, Vec<i64>) {
        let mut list_a = Vec::new();
        let mut list_b = Vec::new();
        for mut ab in content.lines().map(|l| l.split_whitespace()) {
            list_a.push(ab.next().unwrap().parse().unwrap());
            list_b.push(ab.next().unwrap().parse().unwrap());
        }
        (list_a, list_b)
    }

    fn solve_part1(input: &str) -> u64 {
        let (mut list_a, mut list_b) = parse_lists(&input);
        let total_diff = sync_lists(&mut list_a, &mut list_b).unwrap();
        total_diff
    }

    fn solve_part2(input: &str) -> i64 {
        let (mut list_a, mut list_b) = parse_lists(&input);
        let score = similarity_score(&list_a, &list_b).unwrap();
        score
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&test_file("input.txt")), 1530215);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&test_file("input.txt")), 26800609);
    }
}

#[cfg(test)]
mod day2 {
    use super::*;
    use lists::*;

    fn test_file(name: &str) -> String {
        read_file_contents(&input_data("day2", name))
    }

    fn parse_reports(content: &str) -> Vec<Vec<i64>> {
        content
            .lines()
            .map(|l| l.split(' ').map(|p| p.parse().unwrap()).collect::<Vec<_>>())
            .collect()
    }

    fn solve_part1(input: &str) -> usize {
        let reports = parse_reports(input);
        let safe_reports = reports::safe_reports(&reports, (1, 3), false).count();
        safe_reports
    }

    fn solve_part2(input: &str) -> usize {
        let reports = parse_reports(input);
        let safe_reports = reports::safe_reports(&reports, (1, 3), true).count();
        safe_reports
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&test_file("input.txt")), 479);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&test_file("input.txt")), 531);
    }
}

#[cfg(test)]
mod day3 {
    use super::*;
    use corruption::uncorrupt;
    use lists::*;

    fn test_file(name: &str) -> String {
        read_file_contents(&input_data("day3", name))
    }

    fn solve_part1(content: &str) -> i64 {
        uncorrupt(&content)
            .unwrap()
            .iter()
            .map(|m| m.product())
            .sum::<i64>()
    }

    fn solve_part2(content: &str) -> i64 {
        uncorrupt(&content)
            .unwrap()
            .iter()
            .filter_map(|m| m.product_checked())
            .sum::<i64>()
    }

    #[test]
    fn part1_example1() {
        assert_eq!(solve_part1(&test_file("example1.txt")), 161);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&test_file("input.txt")), 171183089);
    }

    #[test]
    fn part2_example2() {
        assert_eq!(solve_part2(&test_file("example2.txt")), 48);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&test_file("input.txt")), 63866497);
    }
}
