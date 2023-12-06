use aoc2023::util;
use std::collections::HashMap;

type CardId = i64;
type Winning = Vec<i64>;
type Owned = Winning;
fn parse_card(line: &str) -> (CardId, Winning, Owned) {
    let (card_name, card) = line.split_once(": ").unwrap();
    let (winning_numbers, my_numbers) = card.split_once("|").unwrap();

    let card_id = util::find_ints(card_name)[0];
    let winning_numbers = util::find_ints(winning_numbers);
    let my_numbers = util::find_ints(my_numbers);
    (card_id, winning_numbers, my_numbers)
}

fn part1(input: &str) -> i64 {
    let score_card = |card: &str| {
        let (_id, winning_numbers, owned_numbers) = parse_card(card);
        let count = owned_numbers
            .iter()
            .filter(|n| winning_numbers.contains(n))
            .count() as u32;
        match count {
            0 => 0,
            _ => 2_i64.pow(count - 1),
        }
    };

    input.lines().map(score_card).sum()
}

fn part2(input: &str) -> usize {
    let mut cards = HashMap::new();

    for line in input.lines() {
        let (card_id, lhs, rhs) = parse_card(line);
        cards.insert(card_id, (lhs, rhs));
    }

    let mut owned_cards: Vec<i64> = cards.keys().cloned().collect();
    let mut cards_to_visit = owned_cards.clone();

    while !cards_to_visit.is_empty() {
        let mut new_cards_to_visit = vec![];
        for card in cards_to_visit {
            let (lhs, rhs) = cards.get(&card).unwrap();
            let next_cards = card + 1..=card + rhs.iter().filter(|n| lhs.contains(n)).count() as i64;
            new_cards_to_visit.extend(next_cards);
        }
        owned_cards.extend(&new_cards_to_visit);
        cards_to_visit = new_cards_to_visit;
    }

    owned_cards.len()
}

fn main() {
    let input = std::fs::read_to_string("./inputs/day-04.txt").unwrap();
    dbg!(part1(&input));
    dbg!(part2(&input));
}

#[test]
fn part1_test() {
    use indoc::indoc;
    let input = indoc! {"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "};
    assert_eq!(part1(&input), 13);
}

#[test]
fn part2_test() {
    use indoc::indoc;
    let input = indoc! {"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "};
    assert_eq!(part2(&input), 30);
}
