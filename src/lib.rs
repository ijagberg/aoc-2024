#![allow(unused)]
use std::{
    fs::File,
    io::{self, BufRead, Read},
};

mod antenna;
mod arcade;
mod blocks;
mod corruption;
mod elephants;
mod guard;
mod hike;
mod lists;
mod pages;
mod regions;
mod reports;
mod stones;
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

    fn solve_part1(content: &str) -> i64 {
        let equations = parse_equations_with_results(content);
        let mut total = 0;

        for (expected_result, equation) in equations {
            if equation
                .all_results(
                    &[Operation::Addition, Operation::Multiplication],
                    expected_result,
                )
                .into_iter()
                .any(|r| r == expected_result)
            {
                total += expected_result;
            }
        }
        total
    }

    fn solve_part2(content: &str) -> i64 {
        let equations = parse_equations_with_results(content);
        let mut total = 0;
        for (expected_result, equation) in equations {
            if equation.can_result_in(
                &[
                    Operation::Addition,
                    Operation::Multiplication,
                    Operation::Concatenation,
                ],
                expected_result,
            ) {
                total += expected_result;
            }
        }

        total
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

#[cfg(test)]
mod day9 {
    use super::*;
    use blocks::FileBlocks;

    fn test_file(name: &str) -> String {
        read_file_contents(&input_data("day9", name))
    }

    fn parse_file_blocks(content: &str) -> FileBlocks {
        let numbers: Vec<usize> = content
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();

        FileBlocks::from_disk_map(&numbers)
    }

    fn solve_part1(content: &str) -> usize {
        let mut fb = parse_file_blocks(content);
        fb.compact_fragmented();
        fb.checksum()
    }

    fn solve_part2(content: &str) -> usize {
        let mut fb = parse_file_blocks(content);
        fb.compact_whole();
        if let Some(s) = fb.get_string_if_possible() {
            println!("{}", s);
        }
        fb.checksum()
    }

    #[test]
    fn part1_example1() {
        assert_eq!(solve_part1(&test_file("example1.txt")), 1928);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&test_file("input.txt")), 6241633730082);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(solve_part2(&test_file("example1.txt")), 2858);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&test_file("input.txt")), 6265268809555);
    }
}

#[cfg(test)]
mod day10 {
    use super::*;
    use hike::TopographyMap;
    use simple_grid::{Grid, GridIndex};

    fn test_file(name: &str) -> String {
        read_file_contents(&input_data("day10", name))
    }

    fn parse_map(content: &str) -> TopographyMap {
        let lines: Vec<_> = content.lines().collect();
        let heights: Vec<u8> = lines
            .iter()
            .map(|l| l.chars())
            .flatten()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();

        TopographyMap::new(Grid::new(lines[0].len(), lines.len(), heights))
    }

    fn solve_part1(content: &str) -> u32 {
        let map = parse_map(content);

        let mut score = 0;
        for th in map.trailheads() {
            let th_score = map.score(th).unwrap();
            score += th_score;
        }

        score
    }

    fn solve_part2(content: &str) -> u32 {
        let map = parse_map(content);

        let mut rating = 0;
        for th in map.trailheads() {
            let th_rating = map.rating(th).unwrap();
            rating += th_rating;
        }

        rating
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&test_file("input.txt")), 646);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&test_file("input.txt")), 1494);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(solve_part2(&test_file("example1.txt")), 81);
    }
}

#[cfg(test)]
mod day11 {
    use super::*;
    use stones::Stones;

    fn test_file(name: &str) -> String {
        read_file_contents(&input_data("day11", name))
    }

    fn parse_stones(content: &str) -> Stones {
        Stones::new(
            &content
                .split(' ')
                .map(|p| p.parse().unwrap())
                .collect::<Vec<_>>(),
        )
    }

    fn solve_part1(content: &str) -> u64 {
        let mut stones = parse_stones(content);

        stones.blink(25)
    }

    fn solve_part2(content: &str) -> u64 {
        let mut stones = parse_stones(content);

        stones.blink(75)
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&test_file("input.txt")), 218956);
    }

    #[test]
    fn part1_example1() {
        assert_eq!(solve_part1(&test_file("example1.txt")), 55312);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&test_file("input.txt")), 259593838049805);
    }
}

#[cfg(test)]
mod day12 {
    use super::*;
    use regions::{Plant, PlantMap};
    use simple_grid::Grid;

    fn test_file(name: &str) -> String {
        read_file_contents(&input_data("day12", name))
    }

    fn parse_plants(content: &str) -> PlantMap {
        let lines: Vec<String> = content
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|l| l.to_owned())
            .collect();
        let data: Vec<Plant> = lines
            .iter()
            .map(|l| l.chars().map(|c| Plant::new(c)))
            .flatten()
            .collect();

        PlantMap::new(Grid::new(lines[0].len(), lines.len(), data))
    }

    fn solve_part1(content: &str) -> u64 {
        let plant_map = parse_plants(content);

        plant_map
            .regions()
            .iter()
            .map(|r| (r.perimeter().len() * r.area()) as u64)
            .sum()
    }

    fn solve_part2(content: &str) -> u64 {
        let plant_map = parse_plants(content);

        plant_map
            .regions()
            .iter()
            .map(|r| (r.sides() * r.area()) as u64)
            .sum()
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&test_file("input.txt")), 1344578);
    }

    #[test]
    fn part1_example1() {
        assert_eq!(solve_part1(&test_file("example1.txt")), 1930);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&test_file("input.txt")), 814302);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(solve_part2(&test_file("example1.txt")), 1206);
    }
}

#[cfg(test)]
mod day13 {
    use arcade::{ArcadeGame, Vec2};

    use super::*;

    fn test_file(name: &str) -> String {
        read_file_contents(&input_data("day13", name))
    }

    fn parse_button(line: &str) -> Vec2 {
        // "34, Y+12"
        // will return (34, 12)
        let (x_value, y_str) = line.split_once(", ").unwrap();
        let y_value = y_str.trim_start_matches("Y+");
        Vec2::new(x_value.parse().unwrap(), y_value.parse().unwrap())
    }

    fn parse_target(line: &str) -> Vec2 {
        // "34, Y=12"
        // will return (34, 12)
        let (x_value, y_str) = line.split_once(", ").unwrap();
        let y_value = y_str.trim_start_matches("Y=");
        Vec2::new(x_value.parse().unwrap(), y_value.parse().unwrap())
    }

    fn parse_arcade_games(content: &str) -> Vec<ArcadeGame> {
        let mut games = Vec::new();
        let lines: Vec<_> = content.lines().collect();
        for i in (0..lines.len()).step_by(4) {
            let a_button_line = lines[i];
            let a_button = parse_button(a_button_line.trim_start_matches("Button A: X+"));

            let b_button_line = lines[i + 1];
            let b_button = parse_button(b_button_line.trim_start_matches("Button B: X+"));

            let target_line = lines[i + 2];
            let target = parse_target(target_line.trim_start_matches("Prize: X="));

            games.push(ArcadeGame::new(target, a_button, b_button));
        }
        games
    }

    fn solve_part1(content: &str) -> u64 {
        let games = parse_arcade_games(content);
        let mut cost = 0;
        for game in games {
            if let Some((na, nb)) = game.win() {
                cost += na * 3 + nb * 1;
            }
        }

        cost
    }

    fn solve_part2(content: &str) -> u64 {
        let games = parse_arcade_games(content);
        let mut cost = 0;
        for mut game in games {
            game.target_mut().x += 10000000000000;
            game.target_mut().y += 10000000000000;
            if let Some((na, nb)) = game.win() {
                cost += na * 3 + nb * 1;
            }
        }

        cost
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&test_file("input.txt")), 33921);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&test_file("input.txt")), 82261957837868);
    }
}
