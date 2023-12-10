use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Hand([char; 5]);
#[derive(Debug, Clone)]
pub struct Bid(u32);

#[aoc_generator(day7)]
pub fn generator(input: &str) -> Vec<(Hand, Bid)> {
    let mut output = vec![];

    for line in input.lines() {
        let mut char_iter = line.chars();
        let hand = Hand([
            // i need https://github.com/rust-lang/rust/issues/81615
            char_iter.next().unwrap(),
            char_iter.next().unwrap(),
            char_iter.next().unwrap(),
            char_iter.next().unwrap(),
            char_iter.next().unwrap(),
        ]);
        char_iter.next(); // skip space
        let bid = Bid(char_iter.collect::<String>().parse().unwrap());

        output.push((hand, bid))
    }

    output
}

// lowest to highest value
const VALUE_ORDER: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

const VALUE_ORDER_WILDCARD: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

#[derive(Debug)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

impl Hand {
    fn second_ordering_score(&self, wildcard: bool) -> u32 {
        let mut score = 0;

        let value_order = &if wildcard {
            VALUE_ORDER_WILDCARD
        } else {
            VALUE_ORDER
        };

        for (i, char) in self.0.iter().rev().enumerate() {
            score += value_order.len().pow(i as u32 + 1)
                * value_order.iter().position(|x| x == char).unwrap()
        }

        score as u32
    }
    fn get_type(&self, wildcard: bool) -> HandType {
        let mut count_map: HashMap<char, u32> = HashMap::new();
        let mut wildcard_count = 0;
        for ch in self.0 {
            if ch == 'J' && wildcard {
                wildcard_count += 1;
                continue;
            }
            count_map.insert(ch, count_map.get(&ch).unwrap_or(&0) + 1);
        }

        let Some(biggest_key_value) = count_map.iter().max_by_key(|x| x.1) else {
            return HandType::FiveOfAKind; // five jokers
        };
        count_map.insert(*biggest_key_value.0, biggest_key_value.1 + wildcard_count);

        if count_map.len() == 1 {
            return HandType::FiveOfAKind;
        }

        let mut count_map_val_iter = count_map.values();
        let occurrences_of_first_unique = *count_map_val_iter.next().unwrap();
        let occurrences_of_second_unique = *count_map_val_iter.next().unwrap();

        if occurrences_of_first_unique == 4 || occurrences_of_second_unique == 4 {
            return HandType::FourOfAKind;
        }

        if (occurrences_of_first_unique == 3 && occurrences_of_second_unique == 2)
            || (occurrences_of_first_unique == 2 && occurrences_of_second_unique == 3)
        {
            return HandType::FullHouse;
        }

        let occurrences_of_third_unique = *count_map_val_iter.next().unwrap();

        if (occurrences_of_first_unique == 3
            && occurrences_of_second_unique == 1
            && occurrences_of_third_unique == 1)
            || (occurrences_of_first_unique == 1
                && occurrences_of_second_unique == 3
                && occurrences_of_third_unique == 1)
            || (occurrences_of_first_unique == 1
                && occurrences_of_second_unique == 1
                && occurrences_of_third_unique == 3)
        {
            return HandType::ThreeOfAKind;
        }

        if count_map.len() == 3 {
            return HandType::TwoPair;
        }

        if count_map.len() == 4 {
            return HandType::OnePair;
        }

        return HandType::HighCard;
    }

    fn cmp_wildcard(&self, other: &Self) -> Ordering {
        let mut ordering = (self.get_type(true) as usize).cmp(&(other.get_type(true) as usize));
        if ordering == Ordering::Equal {
            ordering = self
                .second_ordering_score(true)
                .cmp(&other.second_ordering_score(true))
        };
        ordering
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut ordering = (self.get_type(false) as usize).cmp(&(other.get_type(false) as usize));
        if ordering == Ordering::Equal {
            ordering = self
                .second_ordering_score(false)
                .cmp(&other.second_ordering_score(false))
        };
        ordering
    }
}

#[aoc(day7, part1)]
pub fn part1(input: &Vec<(Hand, Bid)>) -> u32 {
    let mut clone = input.to_vec();
    clone.sort_by(|a, b| a.0.cmp(&b.0));
    clone
        .iter()
        .enumerate()
        .map(|(i, x)| (i + 1) as u32 * x.1 .0)
        .sum()
}

#[aoc(day7, part2)]
pub fn part2(input: &Vec<(Hand, Bid)>) -> u32 {
    let mut clone = input.to_vec();
    clone.sort_by(|a, b| a.0.cmp_wildcard(&b.0));
    clone
        .iter()
        .enumerate()
        .map(|(i, x)| (i + 1) as u32 * x.1 .0)
        .sum()
}
