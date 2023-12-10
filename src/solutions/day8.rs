use std::collections::HashMap;

enum Direction {
    Left,
    Right,
}

pub struct Map {
    directions: Vec<Direction>,
    map: HashMap<String, (String, String)>,
}

#[aoc_generator(day8)]
pub fn generator(input: &str) -> Map {
    let mut lines_iter = input.lines();
    let directions = lines_iter
        .next()
        .unwrap()
        .chars()
        .map(|x| match x {
            'R' => Direction::Right,
            'L' => Direction::Left,
            _ => panic!("direction contains char other than R or L"),
        })
        .collect::<Vec<_>>();

    lines_iter.next(); // skip space between directions and map

    let mut map = HashMap::new();
    for line in lines_iter {
        let mut char_iter = line.chars();
        let key: String = char_iter.by_ref().take(3).collect();
        char_iter.nth(3); // skip 4
        let val0: String = char_iter.by_ref().take(3).collect();
        char_iter.nth(1); // skip 2
        let val1: String = char_iter.by_ref().take(3).collect();

        map.insert(key, (val0, val1));
    }

    Map { directions, map }
}

#[aoc(day8, part1)]
pub fn part1(input: &Map) -> u32 {
    let mut current_address = "AAA";
    let mut steps = 0;

    for (i, direction) in input.directions.iter().cycle().enumerate() {
        if current_address == "ZZZ" {
            steps = i;
            break;
        }
        let ways = input.map.get(current_address).unwrap();
        current_address = match direction {
            Direction::Left => &ways.0,
            Direction::Right => &ways.1,
        }
    }

    steps as u32
}

// thank you chatgpt for gcd and lcm functions
fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
fn lcm_of_vector(numbers: &Vec<usize>) -> usize {
    if numbers.is_empty() {
        return 0;
    }

    let mut result = numbers[0];

    for &number in numbers.iter().skip(1) {
        result = (result * number) / gcd(result, number);
    }

    result
}

#[aoc(day8, part2)]
pub fn part2(input: &Map) -> usize {
    let mut starting_nodes: Vec<String> = input
        .map
        .keys()
        .filter(|x| x.ends_with('A'))
        .map(|x| x.to_owned())
        .collect();

    let mut ending_node_steps = vec![];

    for (i, direction) in input.directions.iter().cycle().enumerate() {
        for node in starting_nodes.iter_mut() {
            if node.ends_with('Z') {
                continue;
            }

            let ways = input.map.get(node).unwrap();
            *node = match direction {
                Direction::Left => ways.0.clone(),
                Direction::Right => ways.1.clone(),
            };

            if node.ends_with('Z') {
                ending_node_steps.push(i + 1)
            }
        }
        if ending_node_steps.len() == starting_nodes.len() {
            break;
        }
    }

    lcm_of_vector(&ending_node_steps)
}
