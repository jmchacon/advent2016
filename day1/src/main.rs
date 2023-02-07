//! day1 advent 20XX
use crate::Direction::{East, North, South, West};
use clap::Parser;
use color_eyre::eyre::Result;
use grid::Location;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use strum_macros::Display;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long, default_value_t = String::from("input.txt"))]
    filename: String,

    #[arg(long, default_value_t = false)]
    debug: bool,
}

#[derive(Clone, Debug, Display, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let mut loc = Location(0, 0);
    let mut dir = North;

    for (line_num, line) in lines.iter().enumerate() {
        let parts = line.split(", ").collect::<Vec<_>>();
        for p in parts.iter().cloned() {
            if args.debug {
                println!("{p}");
            }
            assert!(
                p.len() > 1,
                "{} - bad direction {p} for line {line}",
                line_num + 1
            );
            let mut chars = p.chars();
            let turn = chars.nth(0).unwrap();
            let num = chars.as_str().parse::<isize>().unwrap();
            match turn {
                'L' => {
                    dir = match dir {
                        North => West,
                        South => East,
                        East => North,
                        West => South,
                    }
                }
                'R' => {
                    dir = match dir {
                        North => East,
                        South => West,
                        East => South,
                        West => North,
                    }
                }
                _ => panic!("{} - bad direction {p} for line {line}", line_num + 1),
            }
            loc = match dir {
                North => Location(loc.0, loc.1 + num),
                South => Location(loc.0, loc.1 - num),
                East => Location(loc.0 + num, loc.1),
                West => Location(loc.0 - num, loc.1),
            };
            if args.debug {
                println!("{loc}");
            }
        }
    }
    println!("part1: {}", Location(0, 0).distance(&loc));
    Ok(())
}
