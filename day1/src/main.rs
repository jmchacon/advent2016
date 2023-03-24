//! day1 advent 20XX
use crate::Direction::{East, North, South, West};
use clap::Parser;
use color_eyre::eyre::Result;
use grid::Location;
use std::collections::HashSet;
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
    let mut visited = HashSet::new();
    let mut part2 = None;

    for (line_num, line) in lines.iter().enumerate() {
        let directions = line.split(", ").collect::<Vec<_>>();
        for p in &directions {
            if args.debug {
                println!("{p}");
            }
            assert!(
                p.len() > 1,
                "{} - bad direction {p} for line {line}",
                line_num + 1
            );
            let mut chars = p.chars();
            let turn = chars.next().unwrap();
            let num = chars.as_str().parse::<isize>()?;
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
            let mut newloc = loc.clone();
            loc = match dir {
                North => {
                    for _ in 0..num {
                        newloc = Location(newloc.0, newloc.1 + 1);
                        check_part2(&mut visited, &newloc, &mut part2, args.debug);
                    }
                    newloc
                }
                South => {
                    for _ in 0..num {
                        newloc = Location(newloc.0, newloc.1 - 1);
                        check_part2(&mut visited, &newloc, &mut part2, args.debug);
                    }
                    newloc
                }
                East => {
                    for _ in 0..num {
                        newloc = Location(newloc.0 + 1, newloc.1);
                        check_part2(&mut visited, &newloc, &mut part2, args.debug);
                    }
                    newloc
                }
                West => {
                    for _ in 0..num {
                        newloc = Location(newloc.0 - 1, newloc.1);
                        check_part2(&mut visited, &newloc, &mut part2, args.debug);
                    }
                    newloc
                }
            };
            if args.debug {
                println!("{dir} - {loc}");
            }
        }
    }
    println!("part1: {}", Location(0, 0).distance(&loc));
    println!("part2: {}", part2.unwrap());
    Ok(())
}

fn check_part2(
    visited: &mut HashSet<Location>,
    newloc: &Location,
    part2: &mut Option<u32>,
    debug: bool,
) {
    if visited.contains(newloc) && part2.is_none() {
        if debug {
            println!("part2: {newloc}");
        }
        *part2 = Some(Location(0, 0).distance(newloc));
    }
    visited.insert(newloc.clone());
}
