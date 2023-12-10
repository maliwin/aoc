use aoc2023::util;

fn diff(v: &[i64]) -> Vec<i64> {
    v.iter()
        .skip(1)
        .zip(v.iter())
        .map(|(n2, n1)| n2 - n1)
        .collect::<Vec<_>>()
}

fn extrapolate(v: &[i64]) -> i64 {
    let dv = diff(&v);
    if dv.iter().any(|&n| n != 0) {
        return v.last().unwrap() + extrapolate(&dv);
    } else {
        return *v.last().unwrap();
    }
}

fn extrapolate_left(v: &[i64]) -> i64 {
    let dv = diff(&v);
    if dv.iter().any(|&n| n != 0) {
        return v.first().unwrap() - extrapolate_left(&dv);
    } else {
        return *v.first().unwrap();
    }
}

fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(util::find_ints)
        .map(|v| dbg!(extrapolate(&v)))
        .sum::<i64>()
}

fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(util::find_ints)
        .map(|v| extrapolate_left(&v))
        .sum::<i64>()
}

fn main() {
    let input = std::fs::read_to_string("./inputs/day-09.txt").unwrap();
    dbg!(part1(&input));
    dbg!(part2(&input));
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};
    use indoc::indoc;
    static INPUT: &str = indoc! {"
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
    "};

    #[test]
    fn part1_test() {
        assert_eq!(part1(INPUT), 114);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(INPUT), 2);
    }
}
