use advent_of_code::prelude::{solve, Day, Part, Unsolved, Year};
use clipboard::{ClipboardContext, ClipboardProvider};
use std::error::Error;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "aoc", about = "let's solve aoc!")]
pub enum AdventOfCode2021Cli {
    Open { year: Year, day: Day },
    Solve { year: Year, day: Day, part: Part },
}

pub use webbrowser::open as open_url;
pub fn copy_to_clipboard(content: String) -> Result<(), Box<dyn Error>> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    ctx.set_contents(content)
}

fn main() {
    match AdventOfCode2021Cli::from_args() {
        AdventOfCode2021Cli::Open { year, day } => {
            match open_url(&format!("https://adventofcode.com/{}/day/{}", year, day)) {
                Ok(_) => println!("opened aoc{}'s day {} in the browser 🙌", year, day),
                Err(_) => eprintln!("could not open browser 🙄"),
            }
        }
        AdventOfCode2021Cli::Solve { year, day, part } => match solve(year, day, part) {
            Ok(answer) => {
                println!(
                    "answer to aoc{}'s day {} part {} is {} 💡",
                    year, day, part, answer
                );
                match copy_to_clipboard(answer) {
                    Ok(_) => println!("copied answer to clipboard 🎉"),
                    Err(_) => eprintln!("could not copy answer to clipboard 🙄"),
                };
            }
            Err(error) => match error {
                Unsolved::Year => eprintln!("aoc{} is not solved yet 🤷", year),
                Unsolved::Day => {
                    eprintln!("aoc{}'s day {} is not solved yet 🤷", year, day)
                }
                Unsolved::Part => eprintln!(
                    "aoc{}'s day {} part {} is not solved yet 🤷",
                    year, day, part
                ),
            },
        },
    }
}
