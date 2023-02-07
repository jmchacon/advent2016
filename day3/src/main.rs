//! day3 advent 20XX
use clap::Parser;
use color_eyre::eyre::Result;
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

    let mut count = 0;
    let mut rows = Vec::new();
    for (line_num, line) in lines.iter().enumerate() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        assert!(parts.len() == 3, "{} - bad line {line}", line_num + 1);

        let a = parts[0].parse::<u32>().unwrap();
        let b = parts[1].parse::<u32>().unwrap();
        let c = parts[2].parse::<u32>().unwrap();

        // If these are lengths of the side of a triangle then 2 sides must
        // be bigger than the third.
        if check_triangle(a, b, c) {
            count += 1;
        }

        rows.push((a, b, c));
    }
    assert!(rows.len() % 3 == 0, "Bad number of rows {}", rows.len());

    let mut part2_count = 0;
    for i in (0..rows.len()).step_by(3) {
        if check_triangle(rows[i].0, rows[i + 1].0, rows[i + 2].0) {
            part2_count += 1;
        }
        if check_triangle(rows[i].1, rows[i + 1].1, rows[i + 2].1) {
            part2_count += 1;
        }
        if check_triangle(rows[i].2, rows[i + 1].2, rows[i + 2].2) {
            part2_count += 1;
        }
    }
    println!("part1: {count}");
    println!("part2: {part2_count}");
    Ok(())
}

fn check_triangle(a: u32, b: u32, c: u32) -> bool {
    a + b > c && a + c > b && b + c > a
}
