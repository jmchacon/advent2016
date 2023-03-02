//! day9 advent 20XX
#![cfg_attr(feature = "cargo-clippy", allow(clippy::unwrap_used))]
use clap::Parser;
use color_eyre::eyre::Result;
use std::fmt::Write;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::str::Chars;

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

    for line in &lines {
        let mut c = line.chars();
        println!("part1: {}", outer(&mut c, false));
        let mut c = line.chars();
        println!("part2: {}", outer(&mut c, true));
    }
    Ok(())
}

fn outer(c: &mut Chars, part2: bool) -> usize {
    let mut sz = 0;
    loop {
        let Some(cur) = c.next() else {
                break;
            };

        // Normally we just copy char by char.
        // Unless this is a (AxB) marker which indicates
        // to copy the next A chars after the marker B times
        // and then continue as before.
        if cur == '(' {
            sz += process(c, part2);
            continue;
        }
        sz += 1;
    }
    sz
}

fn process(c: &mut Chars, part2: bool) -> usize {
    let mut out = String::new();
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
    if part2 && out.contains('(') {
        let mut c = out.chars();
        return outer(&mut c, part2);
    }
    out.len()
}
