use aoc2023::util;

fn solve(time: i64, record: i64) -> (i64, i64) {
    let discriminant = ((time * time - 4 * record) as f64).sqrt();
    let l = 0.5 * (time as f64 - discriminant);
    let r = 0.5 * (time as f64 + discriminant);
    ((l + 1.0).floor() as i64, (r - 1.0).ceil() as i64)
}

fn part1(input: &str) -> i64 {
    let lines = input.lines().collect::<Vec<_>>();
    let (times, distances) = (util::find_ints(lines[0]), util::find_ints(lines[1]));

    times
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| solve(*t, *d))
        .map(|(l, r)| r - l + 1)
        .product()
}

fn part2(input: &str) -> i64 {
    part1(&input.replace(" ", ""))
}

fn main() {
    let input = std::fs::read_to_string("./inputs/day-06.txt").unwrap();
    dbg!(part1(&input));
    dbg!(part2(&input));
}

#[test]
fn part1_test() {
    use indoc::indoc;
    let input = indoc! {"
        Time:      7  15   30
        Distance:  9  40  200
    "};
    assert_eq!(part1(&input), 288);
}

#[test]
fn part2_test() {
    use indoc::indoc;
    let input = indoc! {"
        Time:      7  15   30
        Distance:  9  40  200
    "};
    assert_eq!(part2(&input), 71503);
}
