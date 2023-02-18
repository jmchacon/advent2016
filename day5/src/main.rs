//! day5 advent 20XX
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

    for line in &lines {
        let mut cur = 0;
        let mut pass = String::new();
        loop {
            let digest = md5::compute(format!("{line}{cur}"));

            match format!("{digest:x}").as_bytes() {
                [b'0', b'0', b'0', b'0', b'0', x, ..] => {
                    if args.debug {
                        println!("Found at {cur} {digest:x}");
                    }
                    pass = format!("{pass}{}", core::str::from_utf8(&[*x]).unwrap());
                    if pass.len() >= 8 {
                        break;
                    }
                }
                _ => {}
            }
            cur += 1;
        }
        println!("part1: {pass}");
    }
    Ok(())
}
