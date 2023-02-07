//! day2 advent 20XX
use crate::Move::{Down, Left, Right, Up};
use clap::Parser;
use color_eyre::eyre::Result;
use std::fmt::Write;
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
}

#[derive(Clone, Debug, Display, PartialEq, Eq)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

// Keypad represents a keypad which looks like:
//
// 1 2 3
// 4 5 6
// 7 8 9
struct Keypad(u32);

impl Keypad {
    fn mv(&mut self, direction: Move) {
        self.0 = match direction {
            Move::Up => match self.0 {
                1 | 2 | 3 => self.0,
                4 => 1,
                5 => 2,
                6 => 3,
                7 => 4,
                8 => 5,
                9 => 6,
                _ => todo!(),
            },
            Move::Down => match self.0 {
                7 | 8 | 9 => self.0,
                1 => 4,
                2 => 5,
                3 => 6,
                4 => 7,
                5 => 8,
                6 => 9,
                _ => todo!(),
            },
            Move::Left => match self.0 {
                1 | 4 | 7 => self.0,
                2 => 1,
                3 => 2,
                5 => 4,
                6 => 5,
                8 => 7,
                9 => 8,
                _ => todo!(),
            },
            Move::Right => match self.0 {
                3 | 6 | 9 => self.0,
                1 => 2,
                2 => 3,
                4 => 5,
                5 => 6,
                7 => 8,
                8 => 9,
                _ => todo!(),
            },
        }
    }
}

// StrangeKeypad represents a keypad which looks like:
//
//     1
//   2 3 4
// 5 6 7 8 9
//   A B C
//     D
struct StrangeKeypad(u32);

impl StrangeKeypad {
    fn mv(&mut self, direction: Move) {
        self.0 = match direction {
            Move::Up => match self.0 {
                1 | 2 | 4 | 5 | 9 => self.0,
                3 => 1,
                6 => 2,
                7 => 3,
                8 => 4,
                10 => 6,
                11 => 7,
                12 => 8,
                13 => 11,
                _ => todo!(),
            },
            Move::Down => match self.0 {
                5 | 9 | 10 | 12 | 13 => self.0,
                1 => 3,
                2 => 6,
                3 => 7,
                4 => 8,
                6 => 10,
                7 => 11,
                8 => 12,
                11 => 13,
                _ => todo!(),
            },
            Move::Left => match self.0 {
                1 | 2 | 5 | 10 | 13 => self.0,
                3 => 2,
                4 => 3,
                6 => 5,
                7 => 6,
                8 => 7,
                9 => 8,
                11 => 10,
                12 => 11,
                _ => todo!(),
            },
            Move::Right => match self.0 {
                1 | 4 | 9 | 12 | 13 => self.0,
                2 => 3,
                3 => 4,
                5 => 6,
                6 => 7,
                7 => 8,
                8 => 9,
                10 => 11,
                11 => 12,
                _ => todo!(),
            },
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let mut kp = Keypad(5);
    let mut strange_kp = StrangeKeypad(5);
    let mut code = String::new();
    let mut part2_code = String::new();
    for (line_num, line) in lines.iter().enumerate() {
        for c in line.chars() {
            match c {
                'U' => {
                    (kp.mv(Up), strange_kp.mv(Up));
                }
                'D' => {
                    (kp.mv(Down), strange_kp.mv(Down));
                }
                'L' => {
                    (kp.mv(Left), strange_kp.mv(Left));
                }
                'R' => {
                    (kp.mv(Right), strange_kp.mv(Right));
                }
                _ => panic!("{} - bad line {line}", line_num + 1),
            }
        }
        write!(code, "{}", kp.0).unwrap();
        write!(part2_code, "{:X}", strange_kp.0).unwrap();
    }
    println!("part1 - {code}");
    println!("part2 - {part2_code}");
    Ok(())
}
