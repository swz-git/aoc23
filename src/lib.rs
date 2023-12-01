extern crate aoc_runner;

#[macro_use]
extern crate aoc_runner_derive;

pub mod solutions {
    automod::dir!("./src/solutions");
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

aoc_lib! { year = 2023 }
