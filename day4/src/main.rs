//! day4 advent 20XX
use clap::Parser;
use color_eyre::eyre::Result;
use std::collections::HashMap;
use std::fmt::Write;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long, default_value_t = String::from("input.txt"))]
    filename: String,

    #[arg(long, default_value_t = false)]
    debug: bool,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let mut real = 0;
    let mut part2 = 0;
    for (line_num, line) in lines.iter().enumerate() {
        let mut names = HashMap::new();

        let mut sector_id = String::new();
        let mut checksum = String::new();
        let mut checksum_on = false;
        for c in line.chars() {
            match c {
                'a'..='z' => {
                    if checksum_on {
                        write!(checksum, "{c}")?;
                    } else {
                        names.entry(c).and_modify(|v| *v += 1).or_insert(1);
                    }
                }
                '0'..='9' => {
                    write!(sector_id, "{c}")?;
                }
                '[' => {
                    checksum_on = true;
                }
                ']' | '-' => {}
                _ => panic!("{} - bad line {line}", line_num + 1),
            }
        }
        if args.debug {
            println!("id: {sector_id}");
            println!("checksum: {checksum}");
            for n in &names {
                println!("name: {n:?}");
            }
        }

        let mut is_real = true;
        let ck = checksum.chars().collect::<Vec<_>>();
        for i in 0..ck.len() - 1 {
            // Don't do anything if both this char and the next aren't in the map
            // we assembled.
            let cur = ck[i];
            let next = ck[i + 1];
            if names.contains_key(&cur) && names.contains_key(&next) {
                // Now check if the current is bigger than the next. If so, golden.
                let x = names.get(&cur).unwrap();
                let y = names.get(&next).unwrap();
                match x.cmp(y) {
                    std::cmp::Ordering::Less => {}
                    std::cmp::Ordering::Equal => {
                        // If instead the 2 chars have equal counts make sure they are in the
                        // checksum in alphabetical order.
                        if args.debug {
                            println!("x: {x} y: {y} c1: {cur} c2: {next} {}", cur < next);
                        }
                        if cur < next {
                            continue;
                        }
                    }
                    std::cmp::Ordering::Greater => continue,
                }
            }
            // If we didn't pass this is a decoy.
            is_real = false;
        }
        if is_real {
            if args.debug {
                println!("real - {line}");
            }
            let id = sector_id.as_str().parse::<u32>()?;
            real += id;

            let mut decrypt = String::new();

            for c in line.chars() {
                match c {
                    'a'..='z' => {
                        let new = std::char::from_u32(
                            (u32::from(c) - u32::from('a') + id) % 26 + u32::from('a'),
                        )
                        .unwrap();
                        write!(decrypt, "{new}")?;
                    }
                    '-' => {
                        write!(decrypt, " ")?;
                    }
                    _ => {
                        write!(decrypt, "{c}")?;
                    }
                }
            }
            if decrypt.starts_with("northpole object storage") {
                part2 = id;
            }
            if args.debug {
                println!("decrypt: {decrypt}");
            }
        }
    }
    println!("part1: {real}");
    println!("part2: {part2}");
    Ok(())
}
