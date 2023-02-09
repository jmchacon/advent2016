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
    for (line_num, line) in lines.iter().enumerate() {
        let mut names = HashMap::new();

        let mut sector_id = String::new();
        let mut checksum = String::new();
        let mut checksum_on = false;
        for c in line.chars() {
            match c {
                'a'..='z' => {
                    if !checksum_on {
                        names.entry(c).and_modify(|v| *v += 1).or_insert(1);
                    } else {
                        write!(checksum, "{c}").unwrap();
                    }
                }
                '0'..='9' => {
                    write!(sector_id, "{c}").unwrap();
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
        for i in 0..checksum.chars().count() - 1 {
            // Don't do anything if both this char and the next aren't in the map
            // we assembled.
            if names.contains_key(&checksum.chars().nth(i).unwrap()) {
                if names.contains_key(&checksum.chars().nth(i + 1).unwrap()) {
                    // Now check if the current is bigger than the next. If so, golden.
                    // NOTE: Using nth() over and over isn't efficient but these are also
                    //       tiny strings...
                    let x = names.get(&checksum.chars().nth(i).unwrap()).unwrap();
                    let y = names.get(&checksum.chars().nth(i + 1).unwrap()).unwrap();
                    if x > y {
                        continue;
                    } else if x == y {
                        // If instead the 2 chars have equal counts make sure they are in the
                        // checksum in alphabetical order.
                        let c1 = checksum.chars().nth(i).unwrap();
                        let c2 = checksum.chars().nth(i + 1).unwrap();
                        if args.debug {
                            println!("x: {x} y: {y} c1: {c1} c2: {c2} {}", c1 < c2);
                        }
                        if c1 < c2 {
                            continue;
                        }
                    }
                }
            }
            // If we didn't pass this is a decoy.
            is_real = false;
        }
        if is_real {
            if args.debug {
                println!("real - {line}");
            }
            let id = sector_id.as_str().parse::<i32>().unwrap();
            real += id;
        }
    }
    println!("part1: {real}");
    Ok(())
}
