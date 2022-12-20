use anyhow::Error;
use std::cmp::{max, min};
use std::collections::HashSet;

const PART1: bool = false;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

impl Position {
    const MIN: Self = Self {
        x: i32::MIN,
        y: i32::MIN,
        z: i32::MIN,
    };
    const MAX: Self = Self {
        x: i32::MAX,
        y: i32::MAX,
        z: i32::MAX,
    };
}

fn main() -> Result<(), Error> {
    let cubes = std::io::stdin()
        .lines()
        .map(|line| {
            let line = line?;
            let tokens = line.split(',').collect::<Vec<_>>();
            let (x, y, z) = match tokens[..] {
                [x, y, z] => (x, y, z),
                _ => return Err(Error::msg("Wrong number of tokens on line")),
            };

            Ok(Position {
                x: x.parse()?,
                y: y.parse()?,
                z: z.parse()?,
            })
        })
        .collect::<Result<HashSet<_>, Error>>()?;

    println!("{cubes:?}");

    let offsets = &[
        [-1, 0, 0],
        [1, 0, 0],
        [0, -1, 0],
        [0, 1, 0],
        [0, 0, -1],
        [0, 0, 1],
    ];

    if PART1 {
        let mut num_adjacent = 0;

        for cube in cubes.iter().copied() {
            // println!("cube => {cube:?}");
            for [xd, yd, zd] in offsets {
                let adjacent_cube = Position {
                    x: cube.x + xd,
                    y: cube.y + yd,
                    z: cube.z + zd,
                };
                // println!("    adjacent => {adjacent_cube:?}");
                if cubes.contains(&adjacent_cube) {
                    num_adjacent += 1;
                }
            }
        }

        println!("num_total = {}", cubes.len() * 6);
        println!("num_hidden = {num_adjacent}");
        println!("num_visible = {}", cubes.len() * 6 - num_adjacent);
    } else {
        let min_val = cubes
            .iter()
            .copied()
            .fold(Position::MAX, |acc, cube| Position {
                x: min(acc.x, cube.x),
                y: min(acc.y, cube.y),
                z: min(acc.z, cube.z),
            });
        let max_val = cubes
            .iter()
            .copied()
            .fold(Position::MIN, |acc, cube| Position {
                x: max(acc.x, cube.x),
                y: max(acc.y, cube.y),
                z: max(acc.z, cube.z),
            });

        println!("min_val = {min_val:?}");
        println!("max_val = {max_val:?}");

        let mut flood_stack = vec![Position {
            x: min_val.x - 1,
            y: min_val.y - 1,
            z: min_val.z - 1,
        }];
        let mut exterior_cells = HashSet::new();

        while let Some(cube) = flood_stack.pop() {
            if !exterior_cells.insert(cube) {
                continue;
            }

            for [xd, yd, zd] in offsets {
                let adjacent_cube = Position {
                    x: cube.x + xd,
                    y: cube.y + yd,
                    z: cube.z + zd,
                };

                if cubes.contains(&adjacent_cube) {
                    continue;
                }

                if adjacent_cube.x < min_val.x - 1
                    || adjacent_cube.x > max_val.x + 1
                    || adjacent_cube.y < min_val.y - 1
                    || adjacent_cube.y > max_val.y + 1
                    || adjacent_cube.z < min_val.z - 1
                    || adjacent_cube.z > max_val.z + 1
                {
                    continue;
                }

                flood_stack.push(adjacent_cube);
            }
        }

        println!("{exterior_cells:?}");

        let mut num_exterior = 0;

        for cube in cubes.iter().copied() {
            // println!("cube => {cube:?}");
            for [xd, yd, zd] in offsets {
                let adjacent_cube = Position {
                    x: cube.x + xd,
                    y: cube.y + yd,
                    z: cube.z + zd,
                };
                // println!("    adjacent => {adjacent_cube:?}");
                if exterior_cells.contains(&adjacent_cube) {
                    num_exterior += 1;
                }
            }
        }

        println!("{num_exterior}");
    }

    Ok(())
}
