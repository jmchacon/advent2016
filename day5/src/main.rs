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
        let (mut pass, mut pass2) = ([0_u8; 8], [255_u8; 8]);
        let (mut pos, mut pos2) = (0, 0);
        let (mut part1, mut part2) = (false, false);
        loop {
            let digest = md5::compute(format!("{line}{cur}"));
            if let [b'0', b'0', b'0', b'0', b'0', x, y, ..] = format!("{digest:x}").as_bytes() {
                // part2 is a little trickier. x is the position for y in part
                // but x must be 0..7 and y must not have been filled in yet.
                // So compute this by subtracting 0x30 off since this is ASCII
                // and the numbers are below the letters so gives us an easy range check.
                let num = usize::from(*x - 0x30);
                if args.debug {
                    println!("Found at {cur} {digest:x} - {num}");
                }
                if pos < pass.len() {
                    pass[pos] = *x;
                    pos += 1;
                    if pos >= pass.len() {
                        part1 = true;
                    }
                }

                if num < 8 && pass2[num] == 255 {
                    if args.debug {
                        println!("part2 found {num} for {digest:x}");
                    }
                    pass2[num] = *y;
                    pos2 += 1;
                }
                if pos2 >= pass2.len() {
                    part2 = true;
                }
            }
            cur += 1;
            if part1 && part2 {
                break;
            }
        }
        println!("part1: {}", core::str::from_utf8(&pass).unwrap());
        println!("part2: {}", core::str::from_utf8(&pass2).unwrap());
    }
    Ok(())
}
