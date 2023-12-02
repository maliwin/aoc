use std::error::Error;

fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut calibration_values: Vec<i64> = vec![];
    for line in input.lines() {
        let first_digit = line.chars().find(|c| c.is_numeric()).ok_or("no digit")?;
        let last_digit = line.chars().rev().find(|c| c.is_numeric()).ok_or("no digit")?;
        let number = format!("{a}{b}", a = first_digit, b = last_digit).parse()?;
        calibration_values.push(number);
    }
    Ok(calibration_values.iter().sum())
}

fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    // brilliant way of handling overlapping numbers (oneight, twone, etc) incoming:
    let input = input
        .replace("one", "oonee")
        .replace("two", "ttwoo")
        .replace("three", "tthreee")
        .replace("four", "ffourr")
        .replace("five", "ffivee")
        .replace("six", "ssixx")
        .replace("seven", "ssevenn")
        .replace("eight", "eeightt")
        .replace("nine", "nninee");
    // sure, I could turn this into a function and have a mapping of str->int, but this is funnier

    let input = input
        .replace("one", "1")
        .replace("two", "2")
        .replace("three", "3")
        .replace("four", "4")
        .replace("five", "5")
        .replace("six", "6")
        .replace("seven", "7")
        .replace("eight", "8")
        .replace("nine", "9");

    part1(&input)
}

fn main() {
    let input = std::fs::read_to_string("./inputs/day-01.txt").unwrap();
    dbg!(part1(&input).unwrap());
    dbg!(part2(&input).unwrap());
}

#[test]
fn part1_test() {
    use indoc::indoc;
    let input = indoc! {
        "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet"
    };
    assert_eq!(part1(&input).unwrap(), 142);
}

#[test]
fn part2_test() {
    use indoc::indoc;
    let input = indoc! {
        "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen"
    };
    assert_eq!(part2(&input).unwrap(), 281);
}
