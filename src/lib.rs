#![allow(unused)]
use std::{
    fs::File,
    io::{self, BufRead, Read},
};

mod antenna;
mod corruption;
mod elephants;
mod guard;
mod lists;
mod pages;
mod reports;
mod word_search;

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

#[cfg(test)]
mod day4 {
    use super::*;
    use simple_grid::Grid;
    use word_search::WordSearch;

    fn test_file(name: &str) -> String {
        read_file_contents(&input_data("day4", name))
    }

    fn make_grid(content: &str) -> WordSearch {
        let mut lines: Vec<_> = content.lines().collect();
        let width = lines[0].len();
        let height = lines.len();
        let cells: Vec<char> = lines.join("").chars().collect();
        WordSearch::new(Grid::new(width, height, cells))
    }

    fn solve_part1(content: &str) -> usize {
        let puzzle = make_grid(content);
        let count = puzzle.find_all_words(&['X', 'M', 'A', 'S']).len();

        count
    }

    fn solve_part2(content: &str) -> usize {
        let puzzle = make_grid(content);
        let count = puzzle.find_all_crosses(&['M', 'A', 'S']).len();

        count
    }

    #[test]
    fn part1_example1() {
        assert_eq!(solve_part1(&test_file("example1.txt")), 18);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&test_file("input.txt")), 2591);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(solve_part2(&test_file("example1.txt")), 9);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&test_file("input.txt")), 1880);
    }
}

#[cfg(test)]
mod day5 {
    use std::collections::{HashMap, HashSet};

    use super::*;
    use pages::PageRules;
    use simple_grid::Grid;
    use word_search::WordSearch;

    fn test_file(name: &str) -> String {
        read_file_contents(&input_data("day5", name))
    }

    fn parse_rules_and_pages(content: &str) -> (PageRules, Vec<Vec<u32>>) {
        let mut lines = content.lines();

        let mut page_rules = HashMap::new();
        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }
            let (a, b) = line.split_once('|').unwrap();
            let (a, b) = (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap());

            page_rules.entry(a).or_insert(HashSet::new()).insert(b);
        }

        let mut page_collection = Vec::new();
        for line in lines {
            let pages = line.split(',').map(|p| p.parse().unwrap()).collect();
            page_collection.push(pages);
        }

        (PageRules::new(page_rules), page_collection)
    }

    fn solve_part1(content: &str) -> u32 {
        let (rules, pages) = parse_rules_and_pages(content);

        pages
            .iter()
            .filter_map(|p| {
                if rules.is_valid(&p) {
                    Some(p[p.len() / 2])
                } else {
                    None
                }
            })
            .sum()
    }

    fn solve_part2(content: &str) -> u32 {
        let (rules, pages) = parse_rules_and_pages(content);

        pages
            .into_iter()
            .filter_map(|p| {
                if !rules.is_valid(&p) {
                    let reordered = rules.reorder(p);
                    Some(reordered[reordered.len() / 2])
                } else {
                    None
                }
            })
            .sum()
    }

    #[test]
    fn part1_example1() {
        assert_eq!(solve_part1(&test_file("example1.txt")), 143);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&test_file("input.txt")), 5087);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(solve_part2(&test_file("example1.txt")), 123);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&test_file("input.txt")), 4971);
    }
}

#[cfg(test)]
mod day6 {
    use std::collections::HashSet;

    use guard::{Cell, Direction, GuardMap};
    use simple_grid::Grid;

    use super::*;

    fn test_file(name: &str) -> String {
        read_file_contents(&input_data("day6", name))
    }

    fn parse_guard_map(content: &str) -> GuardMap {
        let lines: Vec<String> = content.lines().map(|l| l.to_owned()).collect();
        let cells = lines
            .iter()
            .flat_map(|l| l.chars())
            .map(|c| match c {
                '.' => Cell::Empty,
                '^' => Cell::Guard(Direction::Up),
                '>' => Cell::Guard(Direction::Right),
                'v' => Cell::Guard(Direction::Down),
                '<' => Cell::Guard(Direction::Left),
                '#' => Cell::Wall,
                c => unreachable!(),
            })
            .collect();

        GuardMap::new(Grid::new(lines[0].len(), lines.len(), cells)).unwrap()
    }

    fn solve_part1(content: &str) -> usize {
        let guard_map = parse_guard_map(content);

        let walk = guard_map.get_guard_walk().unwrap();

        let distinct: HashSet<_> = walk.into_iter().map(|(i, _)| i).collect();
        distinct.len()
    }

    fn solve_part2(content: &str) -> usize {
        let guard_map = parse_guard_map(content);
        let obstacles = guard_map.get_obstacle_places();
        dbg!(&obstacles);

        obstacles.len()
    }

    #[test]
    fn part1_example1() {
        assert_eq!(solve_part1(&test_file("example1.txt")), 41);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&test_file("input.txt")), 5101);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(solve_part2(&test_file("example1.txt")), 6);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&test_file("input.txt")), 1951);
    }
}

#[cfg(test)]
mod day7 {
    use super::*;
    use elephants::{Equation, Operation};

    fn test_file(name: &str) -> String {
        read_file_contents(&input_data("day7", name))
    }

    fn parse_equations_with_results(content: &str) -> Vec<(i64, Equation)> {
        let mut v = Vec::new();
        for line in content.lines() {
            let (result, numbers) = line.split_once(':').unwrap();
            let (result, numbers) = (
                result.parse::<i64>().unwrap(),
                Equation::new(
                    numbers
                        .trim()
                        .split(' ')
                        .map(|p| p.parse().unwrap())
                        .collect(),
                ),
            );
            v.push((result, numbers));
        }

        v
    }

    fn solve(content: &str, operations: &[Operation]) -> i64 {
        let equations = parse_equations_with_results(content);
        let mut total = 0;

        for (expected_result, equation) in equations {
            if equation
                .all_results(&operations, expected_result)
                .into_iter()
                .any(|r| r == expected_result)
            {
                total += expected_result;
            }
        }
        total
    }

    fn solve_part1(content: &str) -> i64 {
        let operations = vec![Operation::Addition, Operation::Multiplication];
        solve(content, &operations)
    }

    fn solve_part2(content: &str) -> i64 {
        let operations = vec![
            Operation::Addition,
            Operation::Multiplication,
            Operation::Concatenation,
        ];
        solve(content, &operations)
    }

    #[test]
    fn part1_example1() {
        assert_eq!(solve_part1(&test_file("example1.txt")), 3749);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&test_file("input.txt")), 10741443549536);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(solve_part2(&test_file("example1.txt")), 11387);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&test_file("input.txt")), 500335179214836);
    }
}

#[cfg(test)]
mod day8 {
    use super::*;
    use antenna::{Antenna, AntennaMap};
    use simple_grid::Grid;

    fn test_file(name: &str) -> String {
        read_file_contents(&input_data("day8", name))
    }

    fn parse_antenna_map(content: &str) -> AntennaMap {
        let lines: Vec<String> = content.lines().map(|l| l.to_owned()).collect();

        let data = lines
            .join("")
            .chars()
            .map(|c| match c {
                '.' => None,
                f => Some(Antenna::new(f)),
            })
            .collect();
        AntennaMap::new(Grid::new(lines[0].len(), lines.len(), data))
    }

    fn solve_part1(content: &str) -> usize {
        let antenna_map = parse_antenna_map(content);
        let antinodes = antenna_map.get_antinodes();

        antinodes.cell_iter().filter(|c| !c.is_empty()).count()
    }

    fn solve_part2(content: &str) -> usize {
        let antenna_map = parse_antenna_map(content);
        let antinodes = antenna_map.get_resonant_antinodes();

        antinodes.cell_iter().filter(|c| !c.is_empty()).count()
    }

    #[test]
    fn part1_example1() {
        assert_eq!(solve_part1(&test_file("example1.txt")), 14);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&test_file("input.txt")), 318);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(solve_part2(&test_file("example1.txt")), 34);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&test_file("input.txt")), 1126);
    }
}
