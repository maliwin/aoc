use std::cmp::Ordering;
use std::collections::HashMap;

// sadge...
// #[repr(u32)]
// #[derive(Eq, PartialEq, PartialOrd, Clone, Debug)]
// enum Card {
//     J = 1,
//     Num(u32),
//     T = 10,
//     // J = 11,
//     Q = 12,
//     K = 13,
//     A = 14,
// }
//
// impl TryFrom<char> for Card {
//     type Error = &'static str;
//
//     fn try_from(value: char) -> Result<Self, Self::Error> {
//         match value {
//             'T' => Ok(Card::T),
//             'J' => Ok(Card::J),
//             'Q' => Ok(Card::Q),
//             'K' => Ok(Card::K),
//             'A' => Ok(Card::A),
//             '2'..='9' => Ok(Card::Num(value.to_digit(10).unwrap())),
//             _ => Err("oops"),
//         }
//     }
// }

type Card = u32;
fn card_value(c: char, jack_as_joker: bool) -> Card {
    match c {
        '2'..='9' => c.to_digit(10).unwrap(),
        'J' => match jack_as_joker {
            false => 11,
            true => 1
        },
        'T' => 10,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!(),
    }
}

#[derive(Eq, PartialEq, PartialOrd, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn hand_type(hand: &[Card], jack_as_joker: bool) -> HandType {
    let mut card_map: HashMap<Card, usize> =
        hand.iter().cloned().fold(HashMap::new(), |mut map, card| {
            *map.entry(card).or_insert(0) += 1;
            map
        });

    let jokers = match jack_as_joker {
        false => 0,
        true => card_map
            .remove(&card_value('J', true))
            .unwrap_or(0),
    };
    let max_card_count = card_map.values().max().unwrap_or(&0);

    match card_map.len() {
        0 | 1 => HandType::FiveOfAKind,
        2 => match max_card_count + jokers {
            4 => HandType::FourOfAKind,
            3 => HandType::FullHouse,
            _ => panic!(),
        },
        3 => match max_card_count + jokers {
            3 => HandType::ThreeOfAKind,
            2 => HandType::TwoPair,
            _ => panic!(),
        },
        4 => HandType::OnePair,
        5 => HandType::HighCard,
        _ => panic!(),
    }
}

fn cmp_hands(h1: &[Card], h2: &[Card], jack_as_joker: bool) -> std::cmp::Ordering {
    let (h1t, h2t) = (hand_type(h1, jack_as_joker), hand_type(h2, jack_as_joker));
    let cmp = h1t.partial_cmp(&h2t).unwrap();
    match cmp {
        Ordering::Equal => {
            for (l, r) in h1.iter().zip(h2.iter()) {
                if l > r {
                    return Ordering::Greater;
                }
                if l < r {
                    return Ordering::Less;
                }
            }
            Ordering::Equal
        }
        _ => cmp,
    }
}

fn solve(input: &str, jack_as_joker: bool) -> i32 {
    let mut hands_and_bids = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(" ").unwrap();
            let hand: Vec<Card> = hand.chars().map(|c| card_value(c, jack_as_joker)).collect();
            (hand, bid.parse::<i32>().unwrap())
        })
        .collect::<Vec<_>>();

    hands_and_bids.sort_by(|(h1, _), (h2, _)| cmp_hands(h1, h2, jack_as_joker));

    hands_and_bids
        .iter()
        .enumerate()
        .map(|(idx, (_, bid))| bid * (idx as i32 + 1))
        .sum::<i32>()
}

fn part1(input: &str) -> i32 {
    solve(input, false)
}

fn part2(input: &str) -> i32 {
    solve(input, true)
}

fn main() {
    let input = std::fs::read_to_string("./inputs/day-07.txt").unwrap();
    dbg!(part1(&input));
    dbg!(part2(&input));
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};
    use indoc::indoc;
    static INPUT: &str = indoc! {"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    "};

    #[test]
    fn part1_test() {
        assert_eq!(part1(INPUT), 6440);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(INPUT), 5905);
    }
}
