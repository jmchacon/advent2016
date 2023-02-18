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

    let mut cols = Vec::new();
    let size = lines.iter().next().unwrap().len();
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
    for c in &cols {
        let best = c
            .iter()
            .map(|(k, v)| (v, k))
            .collect::<Vec<_>>()
            .iter()
            .max()
            .unwrap()
            .clone();
        out.push(*best.1);
    }
    println!("part1: {}", core::str::from_utf8(&out).unwrap());
    Ok(())
}
