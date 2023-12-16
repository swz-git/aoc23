crate::solution!(11, parser generator, part1, part2);

#[derive(Debug, Clone)]
pub struct Matrix<T>(Vec<Vec<T>>);

#[derive(Debug, PartialEq, Eq, Clone)]
struct Point {
    x: usize,
    y: usize,
}

pub fn generator(input: &str) -> Matrix<bool> {
    // true is a # which means a galaxy, false is empty space
    Matrix(
        input
            .lines()
            .map(|x| x.chars().map(|x| x == '#').collect())
            .collect(),
    )
}

// fn print_matrix(matrix: &Matrix<bool>) {
//     for row in matrix.0.iter() {
//         println!(
//             "{}",
//             row.iter()
//                 .map(|x| match x {
//                     true => '#',
//                     false => '.',
//                 })
//                 .collect::<String>()
//         )
//     }
// }

pub fn part1(input: &Matrix<bool>) -> u32 {
    let mut empty_rows = vec![];
    for (y, row) in input.0.iter().enumerate() {
        let is_empty = row.iter().all(|pixel| *pixel == false);
        if is_empty {
            empty_rows.push(y);
        }
    }

    let mut empty_cols = vec![];
    for x in 0..input.0[0].len() {
        let mut col = input.0.iter().map(|row| row[x]);
        let is_empty = col.all(|pixel| pixel == false);
        if is_empty {
            empty_cols.push(x);
        }
    }

    let mut expanded_input = input.clone();

    let mut row_inserts = 0; // offset because of previous inserts
    for row_y in empty_rows {
        expanded_input
            .0
            .insert(row_y + row_inserts, input.0[row_y].clone());
        row_inserts += 1;
    }

    let mut col_inserts = 0; // offset because of previous inserts
    for col_x in empty_cols {
        expanded_input.0.iter_mut().for_each(|row| {
            row.insert(col_x + col_inserts, false);
        });

        col_inserts += 1;
    }

    let galaxy_points: Vec<Point> = expanded_input
        .0
        .iter()
        .enumerate()
        .filter_map(|(y, row)| {
            let filtered_row = row
                .iter()
                .enumerate()
                .filter_map(|(x, val)| match val {
                    true => Some(Point { x, y }),
                    false => None,
                })
                .collect::<Vec<Point>>();
            if filtered_row.len() == 0 {
                None
            } else {
                Some(filtered_row)
            }
        })
        .flatten()
        .collect();

    let mut distance_sum = 0;

    for (i, galaxy) in galaxy_points.iter().enumerate() {
        for other in galaxy_points.iter().skip(i) {
            if galaxy == other {
                continue;
            }
            let dist = galaxy.x.abs_diff(other.x) + galaxy.y.abs_diff(other.y);
            distance_sum += dist;
        }
    }

    distance_sum as u32
}

// 1301710448 too low
// 426503472752 also too low
pub fn part2(input: &Matrix<bool>) -> u64 {
    let mut empty_rows = vec![];
    for (y, row) in input.0.iter().enumerate() {
        let is_empty = row.iter().all(|pixel| *pixel == false);
        if is_empty {
            empty_rows.push(y);
        }
    }

    let mut empty_cols = vec![];
    for x in 0..input.0[0].len() {
        let mut col = input.0.iter().map(|row| row[x]);
        let is_empty = col.all(|pixel| pixel == false);
        if is_empty {
            empty_cols.push(x);
        }
    }

    let galaxy_points: Vec<Point> = input
        .0
        .iter()
        .enumerate()
        .filter_map(|(y, row)| {
            let actual_y = y + empty_rows.iter().filter(|er| **er < y).count() * (1_000_000 - 1);
            let filtered_row = row
                .iter()
                .enumerate()
                .filter_map(|(x, val)| {
                    let actual_x =
                        x + empty_cols.iter().filter(|ec| **ec < x).count() * (1_000_000 - 1);
                    match val {
                        true => Some(Point {
                            x: actual_x,
                            y: actual_y,
                        }),
                        false => None,
                    }
                })
                .collect::<Vec<Point>>();
            if filtered_row.len() == 0 {
                None
            } else {
                Some(filtered_row)
            }
        })
        .flatten()
        .collect();

    let mut distance_sum = 0;

    for (i, galaxy) in galaxy_points.iter().enumerate() {
        for other in galaxy_points.iter().skip(i) {
            if galaxy == other {
                continue;
            }
            let dist = galaxy.x.abs_diff(other.x) + galaxy.y.abs_diff(other.y);
            distance_sum += dist;
        }
    }

    distance_sum as u64
}
