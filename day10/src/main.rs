//! day10 advent 20XX
use clap::Parser;
use color_eyre::eyre::Result;
use std::collections::HashMap;
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

    #[arg(long, default_value_t = 17)]
    compare1: usize,

    #[arg(long, default_value_t = 61)]
    compare2: usize,
}

#[derive(Clone, Debug, Display, Eq, Ord, PartialEq, PartialOrd)]
enum Dest {
    Bot(usize),
    Output(usize),
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Instruction {
    bot: usize,
    low: Dest,
    high: Dest,
}

#[allow(clippy::too_many_lines)]
fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let mut instructions = Vec::new();
    let mut bots = HashMap::new();
    let mut outputs = HashMap::new();
    for (line_num, line) in lines.iter().enumerate() {
        let vals = line.split_whitespace().collect::<Vec<_>>();
        match *vals.first().unwrap() {
            "value" => {
                assert_eq!(vals.len(), 6, "{} - bad line {line}", line_num + 1);
                let bot = vals[5].parse::<usize>()?;
                let val = vals[1].parse::<usize>()?;
                bots.entry(bot)
                    .and_modify(|v: &mut Vec<usize>| v.push(val))
                    .or_insert(vec![val]);
                bots.get_mut(&bot).unwrap().sort_unstable();
            }
            "bot" => {
                assert_eq!(vals.len(), 12, "{} - bad line {line}", line_num + 1);
                let l = vals[6].parse::<usize>()?;
                let low = match *vals.get(5).unwrap() {
                    "bot" => Dest::Bot(l),
                    "output" => Dest::Output(l),
                    _ => panic!("{} - bad line {line}", line_num + 1),
                };
                let h = vals[11].parse::<usize>()?;
                let high = match *vals.get(10).unwrap() {
                    "bot" => Dest::Bot(h),
                    "output" => Dest::Output(h),
                    _ => panic!("{} - bad line {line}", line_num + 1),
                };
                instructions.push(Instruction {
                    bot: vals[1].parse()?,
                    low,
                    high,
                });
            }
            _ => panic!("{} - bad line {line}", line_num + 1),
        }
    }
    // The specification doesn't say all bots are covered but the data has
    // N entries and 0..N bots so sorting this just means easy lookup for a
    // given bot. If there were gaps would have used a map instead.
    instructions.sort();
    if args.debug {
        println!("bots:");
        for b in &bots {
            println!("{b:?}");
        }
        println!();
        println!("instructions:");
        for i in &instructions {
            println!("bot: {} lo: {:?} high: {:?}", i.bot, i.low, i.high);
        }
    }

    let (mut found_part1, mut found_part2) = (false, false);
    loop {
        let mut h = 0;
        let mut l = 0;
        let mut i = &Instruction {
            bot: 0,
            low: Dest::Bot(0),
            high: Dest::Bot(0),
        };
        let mut found = false;
        for b in &mut bots {
            if b.1.len() == 2 {
                i = instructions.get(*b.0).unwrap();
                h = b.1.pop().unwrap();
                l = b.1.pop().unwrap();
                found = true;
                break;
            }
        }
        assert!(found, "Can't find work");
        if args.debug {
            println!("{} has ({l},{h}) with {i:?}", i.bot);
        }
        for val in [(&i.low, l), (&i.high, h)] {
            match val.0 {
                Dest::Bot(bot) => {
                    bots.entry(*bot)
                        .and_modify(|v: &mut Vec<usize>| v.push(val.1))
                        .or_insert(vec![val.1]);
                    bots.get_mut(bot).unwrap().sort_unstable();
                }
                Dest::Output(o) => {
                    outputs
                        .entry(*o)
                        .and_modify(|v: &mut Vec<usize>| v.push(val.1))
                        .or_insert(vec![val.1]);
                }
            }
        }
        if !found_part1 {
            for b in &bots {
                if b.1.len() == 2
                    && *b.1.first().unwrap() == args.compare1
                    && *b.1.get(1).unwrap() == args.compare2
                {
                    println!("part1 - bot {} compares", b.0);
                    if args.debug {
                        println!("outputs:");
                        println!("{outputs:?}");
                    }
                    found_part1 = true;
                    break;
                }
            }
        }

        if !found_part2
            && outputs.contains_key(&0)
            && outputs.contains_key(&1)
            && outputs.contains_key(&2)
        {
            let tot = outputs.get(&0).unwrap().first().unwrap()
                * outputs.get(&1).unwrap().first().unwrap()
                * outputs.get(&2).unwrap().first().unwrap();
            println!("part2 - {tot}");
            found_part2 = true;
        }
        if found_part1 && found_part2 {
            break;
        }
    }
    Ok(())
}
