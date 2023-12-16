crate::solution!(9, parser generator, part1, part2);

// i need to wrap the vecs for cargo-aoc to be happy
pub struct SensorOutput(Vec<Vec<i32>>);

pub fn generator(input: &str) -> SensorOutput {
    SensorOutput(
        input
            .lines()
            .map(|x| x.split(" ").map(|x| x.parse().unwrap()).collect())
            .collect(),
    )
}

pub fn part1(input: &SensorOutput) -> i32 {
    let mut sums = 0;

    for value_history in &input.0 {
        let mut value_history = value_history.clone();
        let mut vh_sum = *value_history.last().unwrap();

        // loop until value_history is an empty vec of zeros
        while value_history.iter().filter(|x| **x != 0).next().is_some() {
            let mut swap = vec![];
            for i in 1..value_history.len() {
                swap.push(value_history[i] - value_history[i - 1])
            }
            vh_sum += swap.last().unwrap();
            value_history = swap;
        }

        sums += vh_sum
    }

    sums
}

pub fn part2(input: &SensorOutput) -> i32 {
    let mut sums = 0;

    for value_history in &input.0 {
        let mut value_history = value_history.clone();
        let mut values = vec![*value_history.first().unwrap()];

        // loop until value_history is an empty vec of zeros
        while value_history.iter().filter(|x| **x != 0).next().is_some() {
            let mut swap = vec![];
            for i in 1..value_history.len() {
                swap.push(value_history[i] - value_history[i - 1])
            }
            values.push(*swap.first().unwrap());
            value_history = swap;
        }

        sums += values.iter().rev().fold(0, |acc, x| x - acc)
    }

    sums
}
