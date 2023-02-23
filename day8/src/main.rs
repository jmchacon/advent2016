//! day8 advent 20XX
#![cfg_attr(feature = "cargo-clippy", allow(clippy::unwrap_used))]
use clap::Parser;
use color_eyre::eyre::Result;
use grid::{Grid, Location};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use strum_macros::Display;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long, default_value_t = String::from("input.txt"))]
    filename: String,

    #[arg(long, default_value_t = false)]
    debug: bool,

    #[arg(long, default_value_t = 50)]
    width: usize,

    #[arg(long, default_value_t = 6)]
    height: usize,
}

#[derive(Clone, Debug, Default, Display, PartialEq, Eq)]
enum Pixel {
    On,
    #[default]
    Off,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let mut grid = Grid::<Pixel>::new(args.width, args.height);
    for (line_num, line) in lines.iter().enumerate() {
        if line.starts_with("rect") {
            let b = line.strip_prefix("rect ").unwrap();
            let cords = b.split_once('x').unwrap();
            let x = cords.0.parse::<isize>().unwrap();
            let y = cords.1.parse::<isize>().unwrap();
            for i in 0..x {
                for j in 0..y {
                    grid.add(&Location(i, j), Pixel::On);
                }
            }
        } else if line.starts_with("rotate column x=") {
            let c = line.strip_prefix("rotate column x=").unwrap();
            let cords = c.split_once(" by ").unwrap();
            let x = cords.0.parse::<isize>().unwrap();
            let count = cords.1.parse::<isize>().unwrap();
            let mut v = vec![Pixel::default(); args.height];
            #[allow(clippy::cast_possible_wrap)]
            for i in 0..args.height as isize {
                match grid.get(&Location(x, i)) {
                    Pixel::On => {
                        #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
                        let new = (i + count) as usize % args.height;
                        v[new] = Pixel::On;
                    }
                    Pixel::Off => {}
                }
            }
            for (pos, p) in v.into_iter().enumerate() {
                #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
                grid.add(&Location(x, pos as isize), p);
            }
        } else if line.starts_with("rotate row y=") {
            let c = line.strip_prefix("rotate row y=").unwrap();
            let cords = c.split_once(" by ").unwrap();
            let y = cords.0.parse::<isize>().unwrap();
            let count = cords.1.parse::<isize>().unwrap();
            let mut v = vec![Pixel::default(); args.width];
            #[allow(clippy::cast_possible_wrap)]
            for i in 0..args.width as isize {
                match grid.get(&Location(i, y)) {
                    Pixel::On => {
                        #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
                        let new = (i + count) as usize % args.width;
                        v[new] = Pixel::On;
                    }
                    Pixel::Off => {}
                }
            }
            for (pos, p) in v.into_iter().enumerate() {
                #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
                grid.add(&Location(pos as isize, y), p);
            }
        } else {
            panic!("{} - bad line {line}", line_num + 1);
        }
        if args.debug {
            print_grid(&grid);
        }
    }
    let on = grid.iter().filter(|x| *x.1 == Pixel::On).count();
    println!("part1: {on}");
    println!("part2:");
    print_grid(&grid);
    Ok(())
}

fn print_grid(grid: &Grid<Pixel>) {
    for lc in grid {
        match lc.1 {
            Pixel::On => print!("#"),
            Pixel::Off => print!("."),
        }

        #[allow(clippy::cast_sign_loss)]
        if lc.0 .0 as usize == grid.width() - 1 {
            println!();
        }
    }
    println!();
}
