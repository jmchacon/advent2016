//! day6 advent 20XX
use clap::Parser;
use color_eyre::eyre::Result;
use std::collections::HashMap;
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

    // Going to track letter occurance by column.
    // This means each line is the same length and we just need a vec of hashes to represent
    // each char in that column.
    let mut cols = Vec::new();
    let size = lines.first().unwrap().len();
    for _ in 0..size {
        cols.push(HashMap::new());
    }
    for (line_num, line) in lines.iter().enumerate() {
        assert!(line.len() == size, "{} - bad line {line}", line_num + 1);
        for (pos, b) in line.bytes().enumerate() {
            cols[pos]
                .entry(b)
                .and_modify(|v| *v += 1)
                .or_insert(1_usize);
        }
    }
    let mut out = Vec::new();
    let mut part2 = Vec::new();
    for c in &cols {
        // Loop over the hash, invert it and then find the max and insert that char.
        let best = c.iter().map(|(k, v)| (v, k)).max().unwrap();
        let worst = c.iter().map(|(k, v)| (v, k)).min().unwrap();
        out.push(*best.1);
        part2.push(*worst.1);
    }
    println!("part1: {}", core::str::from_utf8(&out)?);
    println!("part2: {}", core::str::from_utf8(&part2)?);
    Ok(())
}
