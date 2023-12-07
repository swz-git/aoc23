use std::{error::Error, ops::Range, str::FromStr};

#[derive(Debug, Clone)]
pub struct Almanac {
    seed_info: Vec<u32>,
    maps: Vec<Map>,
}
#[derive(Debug, Clone)]
pub struct Map(Vec<MapPart>);
#[derive(Debug, Clone)]
pub struct MapPart(u32, u32, u32);

impl FromStr for Almanac {
    type Err = Box<dyn Error>;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut input_iter = input.split("\n\n");
        let seed_info = input_iter
            .next()
            .ok_or("parsing error")?
            .strip_prefix("seeds: ")
            .ok_or("parsing error")?
            .split(" ")
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let mut maps: Vec<Map> = vec![];

        for map in input_iter {
            let mut map_iter = map.lines();
            map_iter.next().ok_or("parsing error")?; // First line is actually useless

            let mut map = Map(vec![]);

            for map_str in map_iter {
                let mut split = map_str.split(" ");
                let map_part = MapPart(
                    split.next().ok_or("parsing error")?.parse()?,
                    split.next().ok_or("parsing error")?.parse()?,
                    split.next().ok_or("parsing error")?.parse()?,
                );
                map.0.push(map_part)
            }

            maps.push(map);
        }

        Ok(Almanac { seed_info, maps })
    }
}

#[aoc_generator(day5)]
pub fn generator(input: &str) -> Almanac {
    input.parse().unwrap()
}

#[aoc(day5, part1)]
pub fn part1(input: &Almanac) -> u32 {
    let mut lowest = u32::MAX;
    for og_seed in &input.seed_info {
        let mut seed = *og_seed;
        for map in &input.maps {
            for map_part in &map.0 {
                let source_range = map_part.1..(map_part.1 + map_part.2);
                if source_range.contains(&seed) {
                    seed = seed - map_part.1 + map_part.0;
                    break;
                }
            }
        }
        if seed < lowest {
            lowest = seed
        }
    }

    lowest
}

#[aoc(day5, part2)]
pub fn part2(input: &Almanac) -> u32 {
    let mut lowest = u32::MAX;

    let mut ranges: Vec<Range<u32>> = input
        .seed_info
        .chunks(2)
        .map(|chunk| chunk[0]..(chunk[0] + chunk[1]))
        .collect();

    for map in &input.maps {
        for map_part in &map.0 {
            let map_part_range = map_part.1..(map_part.1 + map_part.2);
            ranges = ranges
                .iter()
                .flat_map(|range| {
                    // https://filebin.swz.works/api/file/F55fT1iwaI4 \/
                    let to_return = if range.end < map_part_range.start
                        || range.start > map_part_range.end
                    {
                        // out of range
                        vec![range.clone()]
                    } else if range.start <= map_part_range.start && map_part_range.end <= range.end
                    {
                        // [>
                        vec![
                            (range.start..map_part.1),
                            (map_part.0..(map_part.0 + map_part.2)),
                            ((map_part.1 + map_part.2)..range.end),
                        ]
                    } else if map_part_range.start <= range.start && range.end <= map_part_range.end
                    {
                        // <]
                        vec![
                            ((range.start - map_part.1 + map_part.0)
                                ..(range.end - map_part.1 + map_part.0)),
                        ]
                    } else if map_part_range.start <= range.start && map_part_range.end <= range.end
                    {
                        //  \
                        // //
                        // \
                        vec![
                            ((range.start - map_part.1 + map_part.0)..(map_part.0 + map_part.2)),
                            ((map_part.1 + map_part.2)..range.end),
                        ]
                    } else {
                        // /
                        // \\
                        //  /
                        vec![
                            (range.start..map_part.1),
                            (map_part.0..(range.end - map_part.1 + map_part.0)),
                        ]
                    };

                    println!("{:?} became {:?} in stage {:?}", range, to_return, map.0);

                    to_return
                })
                .collect();
        }
        println!()
    }

    lowest
}
