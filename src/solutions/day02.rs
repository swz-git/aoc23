use std::cmp;

use regex::Regex;

crate::solution!(2, parser generator, part1, part2);

#[derive(Debug)]
struct Subset {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    subsets: Vec<Subset>,
}

fn generator(input: &str) -> Vec<Game> {
    let game_id_regex = Regex::new(r"Game (\d+):").unwrap();
    let subset_regex = Regex::new(r"(\d+ (red|green|blue)(, ))*\d+ (red|green|blue)").unwrap();
    let cube_count_regex = Regex::new(r"(\d+) (red|green|blue)").unwrap();

    input
        .lines()
        .map(|line| {
            let line = line.to_owned();

            let game_id: u32 = game_id_regex
                .captures(&line)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse()
                .unwrap();

            let subset_vec: Vec<Subset> = subset_regex
                .find_iter(&line)
                .filter_map(|x| x.as_str().parse::<String>().ok())
                .map(|subset_str| {
                    let mut subset = Subset {
                        red: 0,
                        green: 0,
                        blue: 0,
                    };
                    cube_count_regex
                        .captures_iter(&subset_str)
                        .for_each(|re_match| {
                            let amount = re_match.get(1).unwrap().as_str().parse::<u32>().unwrap();
                            let color = re_match.get(2).unwrap().as_str();
                            match color {
                                "red" => {
                                    subset.red = amount;
                                }
                                "green" => {
                                    subset.green = amount;
                                }
                                "blue" => {
                                    subset.blue = amount;
                                }
                                _ => { /* panic? */ }
                            }
                        });
                    subset
                })
                .collect();

            Game {
                id: game_id,
                subsets: subset_vec,
            }
        })
        .collect::<Vec<Game>>()
}

fn part1(input: &Vec<Game>) -> u32 {
    let mut sum = 0;

    let limits = Subset {
        red: 12,
        green: 13,
        blue: 14,
    };

    for game in input {
        let mut possible = true;
        for subset in &game.subsets {
            if subset.red > limits.red || subset.green > limits.green || subset.blue > limits.blue {
                possible = false
            }
        }
        if possible {
            sum += game.id
        }
    }

    sum
}

fn part2(input: &Vec<Game>) -> u32 {
    let mut total_power = 0;

    for game in input {
        let mut highest = Subset {
            red: 0,
            green: 0,
            blue: 0,
        };
        for subset in &game.subsets {
            highest.red = cmp::max(highest.red, subset.red);
            highest.green = cmp::max(highest.green, subset.green);
            highest.blue = cmp::max(highest.blue, subset.blue);
        }
        total_power += highest.red * highest.green * highest.blue
    }

    total_power
}
