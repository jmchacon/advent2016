//! day9 advent 20XX
#![cfg_attr(feature = "cargo-clippy", allow(clippy::unwrap_used))]
use clap::Parser;
use color_eyre::eyre::Result;
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

    let mut out = String::new();
    for line in &lines {
        let mut c = line.chars();
        loop {
            let Some(cur) = c.next() else {
                break;
            };

            // Normally we just copy char by char.
            // Unless this is a (AxB) marker which indicates
            // to copy the next A chars after the marker B times
            // and then continue as before.
            if cur == '(' {
                let mut n = String::new();
                // Find the number to copy.
                loop {
                    let cur = c.next().unwrap();
                    if cur == 'x' {
                        break;
                    }
                    write!(n, "{cur}").unwrap();
                }
                let num = n.parse::<usize>().unwrap();
                n.truncate(0);
                // Find the repeat count
                loop {
                    let cur = c.next().unwrap();
                    if cur == ')' {
                        break;
                    }
                    write!(n, "{cur}").unwrap();
                }
                let repeat = n.parse::<usize>().unwrap();
                n.truncate(0);

                // Now pull in the next num chars and save them
                // into a string.
                for _ in 0..num {
                    let cur = c.next().unwrap();
                    write!(n, "{cur}").unwrap();
                }
                // Copy that string repeat times into the main output string.
                for _ in 0..repeat {
                    write!(out, "{n}").unwrap();
                }
                continue;
            }
            write!(out, "{cur}").unwrap();
        }
        if args.debug {
            println!("{line} - {out}");
        }
        println!("part1: {}", out.len());
        out.truncate(0);
    }
    Ok(())
}
