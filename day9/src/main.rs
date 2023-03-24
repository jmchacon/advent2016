//! day9 advent 20XX
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
        println!("part1: {}", outer(&mut c, false, args.debug)?);
        let mut c = line.chars();
        println!("part2: {}", outer(&mut c, true, args.debug)?);
    }
    Ok(())
}

fn outer(c: &mut Chars, part2: bool, debug: bool) -> Result<usize> {
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
            sz += process(c, part2, debug)?;
            continue;
        }
        sz += 1;
    }
    Ok(sz)
}

fn process(c: &mut Chars, part2: bool, debug: bool) -> Result<usize> {
    let mut n = String::new();
    // Find the number to copy.
    loop {
        let cur = c.next().unwrap();
        if cur == 'x' {
            break;
        }
        write!(n, "{cur}")?;
    }
    let num = n.parse::<usize>().unwrap();
    n.truncate(0);
    // Find the repeat count
    loop {
        let cur = c.next().unwrap();
        if cur == ')' {
            break;
        }
        write!(n, "{cur}")?;
    }
    let repeat = n.parse::<usize>().unwrap();

    // Now pull in the next num chars and save them
    // into a string.
    n.truncate(0);
    for _ in 0..num {
        let cur = c.next().unwrap();
        write!(n, "{cur}")?;
    }
    if debug {
        println!("Repeat {repeat} for string of length {}", n.len());
    }
    // Either add up the strings or recurse N times.
    if part2 && n.contains('(') {
        let mut c = n.chars();
        let sz = outer(&mut c, part2, debug)?;
        return Ok(repeat * sz);
    }
    Ok(repeat * n.len())
}
