//! day7 advent 20XX
#![cfg_attr(feature = "cargo-clippy", allow(clippy::unwrap_used))]
use clap::Parser;
use color_eyre::eyre::Result;
use std::collections::HashSet;
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
    let mut ssl_cnt = 0;
    for line in &lines {
        let tls = is_tls(line.as_bytes(), args.debug);
        let ssl = is_ssl(line.as_bytes(), args.debug);
        if tls {
            cnt += 1;
        }
        if ssl {
            ssl_cnt += 1;
        }
        if args.debug {
            println!("{line} TLS - {tls}");
            println!("{line} SSL - {ssl}");
        }
    }
    println!("part1: {cnt}");
    println!("part2: {ssl_cnt}");
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
            if let Some(start) = hypernet {
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

fn is_ssl(c: &[u8], debug: bool) -> bool {
    // ABA = letter, distinct, letter
    // BAB = distinct, letter, distinct
    //
    // Find all the ABA and put in a hash
    // Find all the BAB and put in a separate hash
    // Loop over aba and see if bab corresponding is in other hash.
    let mut abas = HashSet::new();
    let mut babs = HashSet::new();

    let mut hypernet = None;
    for i in 0..c.len() - 2 {
        if c[i] == b'[' {
            hypernet = Some(i);
            continue;
        }
        if c[i] == b']' {
            hypernet = None;
            continue;
        }
        if c[i] == c[i + 2] && c[i] != c[i + 1] {
            let s = core::str::from_utf8(&c[i..i + 3]).unwrap();
            if hypernet.is_some() {
                babs.insert(s);
            } else {
                abas.insert(s);
            }
        }
    }
    if debug {
        for a in &abas {
            println!("ABA: {a}");
        }
        for b in &babs {
            println!("BAB: {b}");
        }
    }
    for a in &abas {
        let b = a.as_bytes();
        if babs.contains(core::str::from_utf8(&[b[1], b[0], b[1]]).unwrap()) {
            return true;
        }
    }
    false
}
