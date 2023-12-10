use regex::Regex;

pub fn find_ints(line: &str) -> Vec<i64> {
    let re = Regex::new(r"-?\d+").unwrap();
    re.find_iter(line)
        .map(|m| m.as_str().parse::<i64>().unwrap())
        .collect()
}
