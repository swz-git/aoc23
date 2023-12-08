use std::collections::HashMap;

#[derive(Debug)]
pub struct Card {
    id: u32,
    winning: Vec<u32>,
    scratched: Vec<u32>,
}

#[aoc_generator(day4)]
pub fn generator(input: &str) -> Vec<Card> {
    let mut cards = vec![];

    for line in input.lines() {
        let mut card = Card {
            id: 0,
            winning: vec![],
            scratched: vec![],
        };

        let mut iter = line.split(' ');

        card.id = iter
            .find(|maybe_id| maybe_id.ends_with(':'))
            .unwrap()
            .strip_suffix(':')
            .unwrap()
            .parse()
            .unwrap();

        for s in iter.by_ref() {
            if s == "|" {
                break;
            }
            if let Ok(num) = s.parse::<u32>() {
                card.winning.push(num);
            }
        }

        for s in iter.by_ref() {
            if let Ok(num) = s.parse::<u32>() {
                card.scratched.push(num);
            }
        }

        cards.push(card)
    }

    cards
}

#[aoc(day4, part1)]
pub fn part1(input: &Vec<Card>) -> u32 {
    let mut sum = 0;
    for card in input {
        let winning_count = card
            .scratched
            .iter()
            .filter(|num| card.winning.contains(num))
            .count();
        sum += 2u32.pow(winning_count as u32 - 1)
    }
    sum
}

#[aoc(day4, part2)]
pub fn part2(input: &Vec<Card>) -> u32 {
    let mut count_map: HashMap<u32, u32> = HashMap::new();

    for card in input {
        count_map.entry(card.id).or_insert(1);
        let winning_count = card
            .scratched
            .iter()
            .filter(|num| card.winning.contains(num))
            .count() as u32;
        for i in (card.id + 1)..(card.id + winning_count + 1) {
            count_map.insert(
                i,
                count_map.get(&i).unwrap_or(&1) + count_map.get(&card.id).unwrap_or(&1),
            );
        }
    }

    count_map.values().sum()
}
