use aoc2023::util;

fn part1() {

}

fn part2() {

}

fn main() {
    dbg!(part1());
    dbg!(part2());
    println!("{}", std::env::current_dir().unwrap().display());
    for line in std::fs::read_to_string("./inputs/test").unwrap().lines() {
        dbg!(line);
    }
    util::test();
}
