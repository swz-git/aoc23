#[derive(Debug)]
pub struct Position(usize, usize);

impl Position {
    fn is_close_to(&self, other: &Self) -> bool {
        self.0.abs_diff(other.0) <= 1 && self.1.abs_diff(other.1) <= 1
    }
}

#[derive(Debug)]
pub struct Number {
    value: usize,
    positions: Vec<Position>,
}

#[derive(Debug)]
pub struct Symbol {
    value: char,
    position: Position,
}

#[derive(Debug)]
pub struct Engine {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

#[aoc_generator(day3)]
pub fn generator(input: &str) -> Engine {
    let mut engine = Engine {
        numbers: vec![],
        symbols: vec![],
    };

    let total_cols = input.lines().next().unwrap().len();

    let mut prev: (String, Vec<Position>) = ("".to_owned(), vec![]);
    let mut line = 0;
    for (i, ch) in input.chars().enumerate() {
        if ch == '\n' {
            line += 1;
            continue;
        }

        let col = i - line * (total_cols + 1);

        if ch.is_digit(10) {
            prev.0 += &ch.to_string();
            prev.1.push(Position(col, line));
            continue;
        }

        if prev.0.len() > 0 {
            engine.numbers.push(Number {
                value: prev.0.parse().unwrap(),
                positions: prev.1,
            });
            prev = ("".to_owned(), vec![]);
        }

        if ch != '.' {
            engine.symbols.push(Symbol {
                value: ch,
                position: Position(col, line),
            })
        }
    }

    engine
}

// I have no regrets to the code below

#[aoc(day3, part1)]
pub fn part1(input: &Engine) -> u32 {
    input
        .numbers
        .iter()
        .filter(|number| {
            number
                .positions
                .iter()
                .find(|digit_pos| {
                    input
                        .symbols
                        .iter()
                        .filter(|sym| sym.position.is_close_to(&digit_pos))
                        .next()
                        .is_some()
                })
                .is_some()
        })
        .fold(0, |acc, num| acc + num.value as u32)
}

#[aoc(day3, part2)]
pub fn part2(input: &Engine) -> u32 {
    input
        .symbols
        .iter()
        .filter(|sym| sym.value == '*')
        .filter_map(|sym| {
            let connected_numbers = input
                .numbers
                .iter()
                .filter(|num| {
                    num.positions
                        .iter()
                        .filter(|num_char_pos| num_char_pos.is_close_to(&sym.position))
                        .next()
                        .is_some()
                })
                .collect::<Vec<&Number>>();
            if connected_numbers.len() != 2 {
                return None;
            }
            Some(connected_numbers[0].value * connected_numbers[1].value)
        })
        .fold(0, |acc, gear_ratio| acc + gear_ratio as u32)
}
