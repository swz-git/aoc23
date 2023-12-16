use std::{
    env, fs,
    path::{Path, PathBuf},
};

use clap::Parser;
use miette::{Context, IntoDiagnostic, MietteDiagnostic, Result};

const YEAR: u32 = 2023;

pub mod solutions {
    automod::dir!(pub "./src/solutions");
}

pub mod runner;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Day
    day: usize,

    /// Input file. If empty, download
    #[arg(short, long)]
    input: Option<PathBuf>,
}

macro_rules! solutions_for_days {
    ($(
       $day_mod_name:ident
    ),*) => {
        [
            $(
                solutions::$day_mod_name::SOLUTION
            ),*
        ]
    };
}

fn main() -> Result<()> {
    let args = Args::parse();

    // try to read `.env`, ignore if fails
    dotenvy::dotenv().map(|_| ()).unwrap_or(());

    let solutions = solutions_for_days!(
        day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
        day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25
    );

    let solution = solutions
        .iter()
        .find(|s| s.day == args.day)
        .ok_or(MietteDiagnostic::new("couldn't find day"))?;

    let day = solution.day;

    let input: String = match args.input {
        Some(path) => fs::read_to_string(path).into_diagnostic()?,
        None => {
            let cache_folder_path = Path::new("./.inputcache");
            let cache_path = cache_folder_path.join(&format!("{day}.txt"));

            if !cache_path.exists() {
                let session_cookie = env::var("AOC_SESSION")
                    .into_diagnostic()
                    .wrap_err("environment variable AOC_SESSION wasn't found")?;

                let input_txt =
                    ureq::get(&format!("https://adventofcode.com/{YEAR}/day/{day}/input"))
                        .set("Cookie", &format!("session={session_cookie}"))
                        .call()
                        .into_diagnostic()
                        .wrap_err("fetching input failed, input may not be available yet")?
                        .into_string()
                        .into_diagnostic()?;

                if !cache_folder_path.exists() {
                    fs::create_dir(cache_folder_path).into_diagnostic()?;
                }

                fs::write(cache_path, &input_txt)
                    .into_diagnostic()
                    .wrap_err("failed to write to input cache")?;

                input_txt
            } else {
                fs::read_to_string(cache_path)
                    .into_diagnostic()
                    .wrap_err("couldn't read cached input")?
            }
        }
    };

    (solution.run)(&input);

    Ok(())
}
