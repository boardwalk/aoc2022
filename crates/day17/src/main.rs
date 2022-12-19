#![feature(generators, generator_trait)]

use anyhow::{Context as _, Error};
use std::fmt;
use std::ops::{Generator, GeneratorState};
use std::pin::Pin;

// IterGenerator

struct IterGenerator<G>(G);

impl<G> Iterator for IterGenerator<G>
where
    G: Generator<Return = ()> + Unpin,
{
    type Item = G::Yield;

    fn next(&mut self) -> Option<Self::Item> {
        match Pin::new(&mut self.0).resume(()) {
            GeneratorState::Yielded(yielded) => Some(yielded),
            GeneratorState::Complete(()) => None,
        }
    }
}

// Pos

#[derive(Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn move_(self, move_: Move) -> Self {
        match move_ {
            Move::Left => Self {
                x: self.x - 1,
                y: self.y,
            },
            Move::Right => Self {
                x: self.x + 1,
                y: self.y,
            },
            Move::Down => Self {
                x: self.x,
                y: self.y - 1,
            },
        }
    }
}

// Move

#[derive(Clone, Copy, Debug)]
enum Move {
    Left,
    Right,
    Down,
}

impl Move {
    fn from_char(ch: char) -> Result<Self, Error> {
        match ch {
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            'v' => Ok(Self::Down),
            _ => Err(Error::msg("Move char is invalid")),
        }
    }
}

// Rock

#[derive(Clone, Copy, Debug)]
struct Rock([u8; 4]);

impl Rock {
    const ALL: [Rock; 5] = [
        // 1 row per u8
        // elem 0 is y = 0, elem 4 is y = 4
        // lsb is x = 0, msb is x = 7
        // shape is aligned to bottom-left
        Rock([0b1111, 0b0000, 0b0000, 0b0000]), // -
        Rock([0b0010, 0b0111, 0b0010, 0b0000]), // +
        Rock([0b0111, 0b0100, 0b0100, 0b0000]), // L backwards
        Rock([0b0001, 0b0001, 0b0001, 0b0001]), // |
        Rock([0b0011, 0b0011, 0b0000, 0b0000]), // box
    ];
}

// Chamber

// 1 row per u8, bottom row first
#[derive(Default)]
struct Chamber(Vec<u8>);

impl fmt::Debug for Chamber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();

        for row in self.0.iter().copied().rev() {
            for x in 0..7 {
                let ch = if row & (1 << x) != 0 { '#' } else { '.' };
                s.push(ch);
            }

            s.push('\n');
        }

        f.write_str(&s)
    }
}

impl Chamber {
    fn height(&self) -> usize {
        self.0.len()
    }

    fn collides(&self, rock: Rock, pos: Pos) -> bool {
        for (yoff, rock_row) in rock.0.iter().copied().enumerate() {
            // shift rock row for position
            // if it would overflow or underflow to wall collision, well, it's a collision
            if pos.x < 0 {
                // rocks are left aligned, so x < 0 will always collide with the left wall
                return true;
            }

            let shifted_rock_row = (rock_row << pos.x) & 0x7f; // chamber is only 7 cells wide ((1 << 7) - 1)

            if (shifted_rock_row >> pos.x) != rock_row {
                // if we clipped any bits off by shifting by the (for sure positive) position, we collided
                return true;
            }

            let chamber_row = self
                .0
                .get((pos.y + yoff as i32) as usize)
                .copied()
                .unwrap_or(0);

            // if chamber row and rock row collide, we collided
            if (chamber_row & shifted_rock_row) != 0 {
                return true;
            }
        }

        // y < 0 always collides (shapes are bottom aligned and they would be hitting the floor)
        pos.y < 0
    }

    fn place(&mut self, rock: Rock, pos: Pos) {
        for (yoff, rock_row) in rock.0.iter().copied().enumerate() {
            // make space in chamber
            let y = (pos.y + yoff as i32) as usize;
            while y >= self.0.len() {
                self.0.push(0);
            }

            let shifted_rock_row = (rock_row << pos.x) & 0x7f; // chamber is only 7 cells wide ((1 << 7) - 1)

            let chamber_row = &mut self.0[y];
            *chamber_row |= shifted_rock_row;
        }

        while let Some(last) = self.0.last().copied() {
            if last != 0 {
                break;
            }

            self.0.pop();
        }
    }
}

fn main() -> Result<(), Error> {
    let moves = std::io::stdin()
        .lines()
        .next()
        .ok_or_else(|| Error::msg("Missing line of input"))?
        .context("Input is not valid UTF-8")?
        .chars()
        .map(Move::from_char)
        .collect::<Result<Vec<_>, Error>>()?;

    // println!("{moves:?}");

    let mut chamber = Chamber::default();
    let mut rock_iter = Rock::ALL.iter().copied().cycle();
    let mut move_iter = moves
        .iter()
        .copied()
        .cycle()
        .map(|m| [m, Move::Down])
        .flatten();

    for _rock_num in 0..2022 {
        let rock = rock_iter.next().unwrap();
        // println!("{rock:?}");

        // "Each rock appears so that its left edge is two units away from the left wall and its bottom edge is three
        // units above the highest rock in the room (or the floor, if there isn't one)."
        let mut pos = Pos {
            x: 2,
            y: chamber.height() as i32 + 3,
        };

        loop {
            let move_ = move_iter.next().unwrap();
            // println!("{move_:?}");

            let new_pos = pos.move_(move_);

            if chamber.collides(rock, new_pos) {
                if matches!(move_, Move::Down) {
                    chamber.place(rock, pos);
                    break;
                }
            } else {
                pos = new_pos;
            }
        }

        // println!("{chamber:?}");
    }

    println!("height of chamber: {}", chamber.height());
    Ok(())
}
