#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::East => Direction::West,
            Direction::West => Direction::East,
            Direction::North => Direction::South,
            Direction::South => Direction::North,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Pipe {
    NorthSouth, // |
    WestEast,   // -
    NorthEast,  // L
    NorthWest,  // J
    SouthWest,  // 7
    SouthEast,  // F
}

impl Pipe {
    fn connections(&self) -> (Direction, Direction) {
        match self {
            Self::NorthSouth => (Direction::North, Direction::South),
            Self::WestEast => (Direction::West, Direction::East),
            Self::NorthEast => (Direction::North, Direction::East),
            Self::NorthWest => (Direction::North, Direction::West),
            Self::SouthWest => (Direction::South, Direction::West),
            Self::SouthEast => (Direction::South, Direction::East),
        }
    }

    /// returns None if connection passed isn't valid connection for self
    fn other(&self, connection: &Direction) -> Option<Direction> {
        let connections = self.connections();
        if &connections.0 == connection {
            return Some(connections.1);
        }
        if &connections.1 == connection {
            return Some(connections.0);
        }
        None
    }

    fn connects_to(&self, connection: &Direction) -> bool {
        let connections = self.connections();
        &connections.0 == connection || &connections.1 == connection
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Tile {
    Pipe(Pipe),       // see above
    Ground,           // .
    StartingPosition, // S
}

#[derive(Debug)]
pub struct ParsePipeError;
impl TryFrom<char> for Tile {
    type Error = ParsePipeError;
    fn try_from(value: char) -> Result<Self, ParsePipeError> {
        match value {
            '|' => Ok(Self::Pipe(Pipe::NorthSouth)),
            '-' => Ok(Self::Pipe(Pipe::WestEast)),
            'L' => Ok(Self::Pipe(Pipe::NorthEast)),
            'J' => Ok(Self::Pipe(Pipe::NorthWest)),
            '7' => Ok(Self::Pipe(Pipe::SouthWest)),
            'F' => Ok(Self::Pipe(Pipe::SouthEast)),
            '.' => Ok(Self::Ground),
            'S' => Ok(Self::StartingPosition),
            _ => Err(ParsePipeError),
        }
    }
}

#[derive(Debug)]
pub struct Matrix<T>(Vec<Vec<T>>);

impl<T> Matrix<T> {
    fn get_value(&self, p: &Point) -> &T {
        &self.0[p.y][p.x]
    }
}

impl Matrix<Tile> {
    fn starting(&self) -> Option<Point> {
        for (y, row) in self.0.iter().enumerate() {
            for (x, val) in row.iter().enumerate() {
                if val == &Tile::StartingPosition {
                    return Some(Point { x: x, y: y });
                }
            }
        }
        None
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn to(&self, direction: &Direction) -> Option<Self> {
        match direction {
            Direction::East => Some(Self {
                x: self.x + 1,
                y: self.y,
            }),
            Direction::West => {
                let Some(x) = self.x.checked_sub(1) else {
                    return None;
                };
                Some(Self { x, y: self.y })
            }
            Direction::North => {
                let Some(y) = self.y.checked_sub(1) else {
                    return None;
                };
                Some(Self { x: self.x, y })
            }
            Direction::South => Some(Self {
                x: self.x,
                y: self.y + 1,
            }),
        }
    }
    fn direction_to(&self, other: &Self) -> Option<Direction> {
        let diff = (
            (other.x as isize - self.x as isize).max(-1).min(1),
            (other.y as isize - self.y as isize).max(-1).min(1),
        );
        match diff {
            (1, 0) => Some(Direction::East),
            (-1, 0) => Some(Direction::West),
            (0, 1) => Some(Direction::South),
            (0, -1) => Some(Direction::North),
            _ => None,
        }
    }
}

#[aoc_generator(day10)]
pub fn generator(input: &str) -> Matrix<Tile> {
    Matrix(
        input
            .lines()
            .map(|x| x.chars().map(|x| x.try_into().unwrap()).collect())
            .collect(),
    )
}

#[aoc(day10, part1)]
pub fn part1(input: &Matrix<Tile>) -> u32 {
    let size = Point {
        x: input.0[0].len(),
        y: input.0.len(),
    };

    let start_tile = input.starting().unwrap();

    let start_poses: Vec<Point> = [
        start_tile.to(&Direction::East),
        start_tile.to(&Direction::West),
        start_tile.to(&Direction::North),
        start_tile.to(&Direction::South),
    ]
    .into_iter()
    .filter_map(|maybe_p| {
        let Some(p) = maybe_p else { return None };
        let Some(dir) = p.direction_to(&start_tile) else {
            return None;
        };
        let Tile::Pipe(pipe) = input.get_value(&p) else {
            return None;
        };
        if pipe.connects_to(&dir) {
            Some(p)
        } else {
            None
        }
    })
    .collect();

    let mut steps_matrix: Matrix<Option<usize>> = Matrix(vec![vec![None; size.x]; size.y]);

    for start_pos in start_poses {
        let start_dir = {
            let Tile::Pipe(start_pipe) = input.get_value(&start_pos) else {
                panic!("tile expected to be pipe")
            };
            [start_pipe.connections().0, start_pipe.connections().1]
                .into_iter()
                .find(|con| start_pos.to(&con).unwrap() != start_tile)
                .unwrap()
        };

        let mut cur_pos = start_pos;
        let mut cur_dir = start_dir;

        for i in 1.. {
            let Tile::Pipe(cur_pipe) = input.get_value(&cur_pos) else {
                break;
            };

            if let Some(cur_step_val) = steps_matrix.0[cur_pos.y][cur_pos.x] {
                if cur_step_val < i {
                    break; // no reason to go backwards
                }
            }

            let to_set = steps_matrix.0[cur_pos.y][cur_pos.x]
                .unwrap_or(usize::MAX)
                .min(i);

            steps_matrix.0[cur_pos.y][cur_pos.x] = Some(to_set);

            if !cur_pipe.connects_to(&cur_dir) {
                break;
            }
            let Tile::Pipe(new_pipe) = input.get_value(&cur_pos.to(&cur_dir).unwrap()) else {
                break;
            };
            if !new_pipe.connects_to(&cur_dir.opposite()) {
                break;
            }

            cur_pos = cur_pos.to(&cur_dir).unwrap();
            cur_dir = new_pipe.other(&cur_dir.opposite()).unwrap();
        }
    }

    steps_matrix
        .0
        .iter()
        .map(|x| x.iter().max())
        .max()
        .unwrap()
        .unwrap()
        .unwrap() as u32
}

#[aoc(day10, part2)]
pub fn part2(input: &Matrix<Tile>) -> u32 {
    let size = Point {
        x: input.0[0].len(),
        y: input.0.len(),
    };

    let start_tile = input.starting().unwrap();

    let start_stuff: Vec<(Point, Direction)> = [
        start_tile.to(&Direction::East),
        start_tile.to(&Direction::West),
        start_tile.to(&Direction::North),
        start_tile.to(&Direction::South),
    ]
    .into_iter()
    .filter_map(|maybe_p| {
        let Some(p) = maybe_p else { return None };
        let Some(dir) = p.direction_to(&start_tile) else {
            return None;
        };
        let Tile::Pipe(pipe) = input.get_value(&p) else {
            return None;
        };
        if pipe.connects_to(&dir) {
            Some((p, dir.opposite()))
        } else {
            None
        }
    })
    .collect();

    let start_poses = start_stuff.clone().into_iter().map(|x| x.0);
    let start_connections: Vec<Direction> = start_stuff.into_iter().map(|x| x.1).collect();

    let mut is_loop_matrix: Matrix<bool> = Matrix(vec![vec![false; size.x]; size.y]);

    is_loop_matrix.0[start_tile.y][start_tile.x] = true;

    for start_pos in start_poses {
        let start_dir = {
            let Tile::Pipe(start_pipe) = input.get_value(&start_pos) else {
                panic!("tile expected to be pipe")
            };
            [start_pipe.connections().0, start_pipe.connections().1]
                .into_iter()
                .find(|con| start_pos.to(&con).unwrap() != start_tile)
                .unwrap()
        };

        let mut cur_pos = start_pos;
        let mut cur_dir = start_dir;

        for _ in 1.. {
            let Tile::Pipe(cur_pipe) = input.get_value(&cur_pos) else {
                break;
            };

            if is_loop_matrix.0[cur_pos.y][cur_pos.x] {
                break;
            }

            is_loop_matrix.0[cur_pos.y][cur_pos.x] = true;

            if !cur_pipe.connects_to(&cur_dir) {
                break;
            }
            let Tile::Pipe(new_pipe) = input.get_value(&cur_pos.to(&cur_dir).unwrap()) else {
                break;
            };
            if !new_pipe.connects_to(&cur_dir.opposite()) {
                break;
            }

            cur_pos = cur_pos.to(&cur_dir).unwrap();
            cur_dir = new_pipe.other(&cur_dir.opposite()).unwrap();
        }
    }

    // Prints out which tiles are apart of the loop
    // for row in &is_loop_matrix.0 {
    //     println!(
    //         "{}",
    //         row.iter()
    //             .map(|x| match x {
    //                 true => "X ",
    //                 _ => ". ",
    //             })
    //             .collect::<String>()
    //     )
    // }

    let mut surrounded_tiles = 0;

    for (y, row) in input.0.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            let is_loop = *is_loop_matrix.get_value(&Point { x, y });
            if is_loop {
                continue;
            }

            let mut intersections = 0;

            // raycast from left to our x
            for ray_x in 0..x {
                let ray_point = Point { x: ray_x, y };
                if !*is_loop_matrix.get_value(&ray_point) {
                    continue;
                }
                let connects_to_south = match input.get_value(&ray_point) {
                    Tile::Pipe(pipe) => pipe.connects_to(&Direction::South),
                    Tile::StartingPosition => start_connections.contains(&Direction::South),
                    _ => false,
                };
                if connects_to_south {
                    intersections += 1
                }
            }

            if intersections % 2 == 1 {
                surrounded_tiles += 1
            }
        }
    }

    surrounded_tiles
}
