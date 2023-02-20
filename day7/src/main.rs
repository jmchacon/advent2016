//! day7 advent 20XX
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

    let mut cnt = 0;
    for line in &lines {
        let tls = is_tls(line.as_bytes(), args.debug);
        if tls {
            cnt += 1;
        }
        if args.debug {
            println!("{line} TLS - {tls}");
        }
    }
    println!("part1: {cnt}");
    Ok(())
}

fn is_tls(c: &[u8], debug: bool) -> bool {
    let mut hypernet = None;
    let mut is_tls = false;
    let mut found = 0;
    for i in 0..c.len() - 3 {
        if c[i] == b'[' {
            hypernet = Some(i);
            continue;
        }
        if c[i] == b']' {
            // If we ever find one in a hypernet block it's always false.
            if hypernet.is_some() {
                let start = hypernet.unwrap();
                let end = i;
                if found >= start && found <= end {
                    return false;
                }
            }
            continue;
        }
        // Found a palindrome which must be distinct chars.
        // xyyx = yes
        // aaaa = no
        if c[i] == c[i + 3] && c[i + 1] == c[i + 2] && c[i] != c[i + 1] {
            if debug {
                println!("Found {}", core::str::from_utf8(&c[i..i + 4]).unwrap());
            }
            found = i;
            is_tls = true;
        }
    }
    is_tls
}
