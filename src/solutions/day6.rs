pub struct Race {
    /// time in ms
    time: u32,
    /// record distance in mm
    record: u32,
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> u32 {
    let mut split = input.split("\n");

    let time_iter = split
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .split(" ")
        .filter_map(|x| x.parse::<u32>().ok());

    let record_iter = split
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .split(" ")
        .filter_map(|x| x.parse::<u32>().ok());

    time_iter
        .zip(record_iter)
        .map(|race| {
            (0..=race.0)
                .map(|x| (race.0 - x) * x)
                .filter(|x| x > &race.1)
                .count()
        })
        .fold(1, |acc, x| acc * x) as u32
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> u32 {
    let mut split = input.split("\n");

    let time: u64 = split
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .replace(" ", "")
        .parse()
        .unwrap();

    // u32 is too small
    let record: u64 = split
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .replace(" ", "")
        .parse()
        .unwrap();

    (0..=time)
        .map(|x| (time - x) * x)
        .filter(|x| x > &record)
        .count() as u32
}
