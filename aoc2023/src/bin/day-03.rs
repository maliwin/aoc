use regex::Regex;
use std::collections::{HashMap, HashSet};

fn find_ints_and_pos(line: &str) -> Vec<(i64, (usize, usize))> {
    let re = Regex::new(r"\d+").unwrap();
    re.find_iter(line)
        .map(|m| (m.as_str().parse().unwrap(), (m.start(), m.end())))
        .collect()
}

fn char_2d(text: &str) -> Vec<Vec<char>> {
    text.lines().map(|l| l.chars().collect()).collect()
}

fn adjacent(input: &Vec<Vec<char>>, x: usize, y: usize) -> HashMap<(usize, usize), char> {
    let mut adjacent = HashMap::new();

    // itertools::iproduct would be nicer
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            };

            let (x, y) = (x as i32 + i, y as i32 + j);
            if x < 0 || y < 0 {
                continue;
            }

            let (x, y) = (x as usize, y as usize);
            if x >= input.len() || y >= input[x].len() {
                continue;
            }

            adjacent.insert((x, y), input[x][y]);
        }
    }

    adjacent
}

fn part1(input: &str) -> i64 {
    let input2d: Vec<Vec<char>> = char_2d(input);
    let mut sum = 0;

    for (i, line) in input.lines().enumerate() {
        for (num, (j_start, j_end)) in find_ints_and_pos(line) {
            let adjacent_chars: HashSet<char> = (j_start..j_end)
                .map(|j| adjacent(&input2d, i, j).into_values())
                .flatten()
                .collect();

            sum += adjacent_chars
                .iter()
                .find_map(|c| match c {
                    '0'..='9' | '.' => None,
                    _ => Some(num),
                })
                .unwrap_or(0);
        }
    }

    sum
}

fn part2(input: &str) -> i64 {
    let input2d = char_2d(input);
    let mut gear_map = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        for (num, (j_start, j_end)) in find_ints_and_pos(line) {
            let adjacent_gear_coords = (j_start..j_end)
                .map(|j| adjacent(&input2d, i, j))
                .flatten()
                .filter_map(|(coords, c)| match c {
                    '*' => Some(coords),
                    _ => None,
                });

            for (x, y) in adjacent_gear_coords {
                gear_map
                    .entry((x, y))
                    .and_modify(|v: &mut HashSet<_>| {
                        v.insert(num);
                    })
                    .or_insert_with(|| HashSet::from([num]));
            }
        }
    }

    gear_map
        .iter()
        .filter_map(|(_, surrounding_nums)| match surrounding_nums.len() {
            2 => Some(surrounding_nums.iter().fold(1, |acc, n| acc * n)),
            _ => None,
        })
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("./inputs/day-03.txt").unwrap();
    dbg!(part1(&input));
    dbg!(part2(&input));
}

#[test]
fn part1_test() {
    use indoc::indoc;
    let input = indoc! {"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
        "};
    assert_eq!(part1(&input), 4361);
}

#[test]
fn part2_test() {
    use indoc::indoc;
    let input = indoc! {"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
        "};
    assert_eq!(part2(&input), 467835);
}
