use anyhow::Error;
use std::cmp::Ordering;
use std::collections::HashSet;

const PART1: bool = false;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

fn move_head(dir: Direction, head: Position) -> Position {
    match dir {
        Direction::Left => Position {
            x: head.x - 1,
            y: head.y,
        },
        Direction::Right => Position {
            x: head.x + 1,
            y: head.y,
        },
        Direction::Up => Position {
            x: head.x,
            y: head.y - 1,
        },
        Direction::Down => Position {
            x: head.x,
            y: head.y + 1,
        },
    }
}

fn offset<T: Ord>(a: &T, b: &T) -> i32 {
    match Ord::cmp(a, b) {
        Ordering::Less => 1,
        Ordering::Greater => -1,
        Ordering::Equal => 0,
    }
}

fn move_tail(head: Position, tail: Position) -> Position {
    if i32::abs_diff(tail.x, head.x) < 2 && i32::abs_diff(tail.y, head.y) < 2 {
        return tail;
    }

    Position {
        x: tail.x + offset(&tail.x, &head.x),
        y: tail.y + offset(&tail.y, &head.y),
    }
}

fn main() -> Result<(), Error> {
    // parse input
    let mut dirs = Vec::new();

    for line in std::io::stdin().lines() {
        let line = line?;
        let tokens = line.split_ascii_whitespace().collect::<Vec<_>>();

        let (dir, count) = match tokens[..] {
            [dir, count] => (dir, count),
            _ => return Err(Error::msg("Invalid number of tokens on line")),
        };

        let dir = match dir {
            "L" => Direction::Left,
            "R" => Direction::Right,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => return Err(Error::msg("Invalid direction on line"))?,
        };

        let count = count
            .parse::<u32>()
            .map_err(|_| Error::msg("Invalid count on line"))?;

        for _ in 0..count {
            dirs.push(dir);
        }
    }

    // simulate positions
    let num_positions = if PART1 { 2 } else { 10 };

    let mut positions: Vec<Position> = Vec::new();
    positions.resize_with(num_positions, Default::default);

    let mut visited = HashSet::new();
    visited.insert(*positions.last().unwrap());

    for dir in &dirs {
        for i in 0..positions.len() {
            positions[i] = match i {
                0 => move_head(*dir, positions[i]),
                _ => move_tail(positions[i - 1], positions[i]),
            };
        }

        visited.insert(*positions.last().unwrap());
    }

    println!("{}", visited.len());
    Ok(())
}
