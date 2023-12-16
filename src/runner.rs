use std::time::{Duration, Instant};

#[macro_export]
macro_rules! solution {
    ($day:expr, parser $parser_func:expr $(, $func:expr )* ) => {
        pub const SOLUTION: crate::runner::Solution = crate::runner::Solution {run, day: $day};
        fn run(input: &str) {
            const DAY: usize = $day;
            println!("AOC Day {}:", DAY);

            let parser_function: fn(&str) -> _ = $parser_func;
            let parser_result: crate::runner::RunResult<_> = crate::runner::run_part(parser_function, input);
            println!(
                "   parser {}:\n      Time taken: {:?}",
                stringify!($parser_func),
                parser_result.duration
            );

            $({
                let function: fn(&_) -> _ = $func;
                let result = crate::runner::run_part(function, &parser_result.result);
                println!(
                    "   {}:\n      Time taken: {:?}\n      Result: {:?}",
                    stringify!($func),
                    result.duration,
                    result.result
                )
            })*
        }
    };
    ($day:expr$(, $func:expr )* ) => {
        pub const SOLUTION: crate::runner::Solution = crate::runner::Solution {run, day: $day};
        #[allow(unused)]
        fn run(input: &str) {
            const DAY: usize = $day;
            println!("AOC Day {}:", DAY);
            $({
                let function: fn(&str) -> _ = $func;
                let result = crate::runner::run_part(function, &input);
                println!(
                    "   {}:\n      Time taken: {:?}\n      Result: {:?}",
                    stringify!($func),
                    result.duration,
                    result.result
                )
            })*
        }
    };
}

pub struct Solution {
    pub day: usize,
    pub run: fn(&str) -> (),
}

pub struct RunResult<T> {
    pub duration: Duration,
    pub result: T,
}

pub fn run_part<TP, TR>(function: fn(TP) -> TR, input: TP) -> RunResult<TR> {
    let before = Instant::now();
    let result = function(input);
    let duration = before.elapsed();

    RunResult { duration, result }
}
