crate::solution!(1, part1, part1_improved, part2, part2_improved);

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .map(|x| {
            x.iter()
                .filter_map(|x| x.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .map(|x| x.first().unwrap() * 10 + x.last().unwrap())
        .sum()
}

fn part1_improved(input: &str) -> u32 {
    let mut nums: Vec<u32> = vec![];

    for line in input.lines() {
        let mut pair = (0, 0);
        for ch in line.chars() {
            let Some(digit) = ch.to_digit(10) else {
                continue;
            };
            pair.0 = digit;
            break;
        }

        for ch in line.chars().rev() {
            let Some(digit) = ch.to_digit(10) else {
                continue;
            };
            pair.1 = digit;
            break;
        }

        nums.push(pair.0 * 10 + pair.1)
    }

    nums.iter().sum()
}

const MAP: &[(&str, &str)] = &[
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9"),
];

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line: &str| {
            let mut line = line.to_owned();
            for i in 0.. {
                if line.len() == i {
                    break;
                }
                for str_int in MAP {
                    if line[i..].starts_with(str_int.0) {
                        line.remove(i);
                        line.insert_str(i, str_int.1);
                    }
                }
            }
            line
        })
        .map(|line| line.chars().collect::<Vec<char>>())
        .map(|line| {
            line.iter()
                .filter_map(|ch| ch.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .map(|line| line.first().unwrap() * 10 + line.last().unwrap())
        .sum()
}

fn part2_improved(input: &str) -> u32 {
    let mut lines: Vec<String> = input.lines().map(|x| x.to_owned()).collect();

    lines.iter_mut().for_each(|line| {
        for i in 0.. {
            if line.len() == i {
                break;
            }
            for str_int in MAP {
                if line[i..].starts_with(str_int.0) {
                    line.remove(i);
                    line.insert_str(i, str_int.1);
                }
            }
        }
    });

    let mut nums: Vec<u32> = vec![];

    for line in lines {
        let mut pair = (0, 0);
        for ch in line.chars() {
            let Some(digit) = ch.to_digit(10) else {
                continue;
            };
            pair.0 = digit;
            break;
        }

        for ch in line.chars().rev() {
            let Some(digit) = ch.to_digit(10) else {
                continue;
            };
            pair.1 = digit;
            break;
        }

        nums.push(pair.0 * 10 + pair.1)
    }

    nums.iter().sum()
}
