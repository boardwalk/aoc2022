#![feature(generators, generator_trait)]

use anyhow::Error;
use std::ops::{Generator, GeneratorState};
use std::pin::Pin;
use std::{cmp, fmt};

const PART1: bool = false;

enum Action {
    Visit(usize, usize),
    Reset,
}

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

fn from_left(width: usize, height: usize) -> impl Iterator<Item = Action> {
    IterGenerator(move || {
        for y in 0..height {
            for x in 0..width {
                yield Action::Visit(x, y)
            }

            yield Action::Reset
        }
    })
}

fn from_right(width: usize, height: usize) -> impl Iterator<Item = Action> {
    IterGenerator(move || {
        for y in 0..height {
            for x in (0..width).rev() {
                yield Action::Visit(x, y)
            }

            yield Action::Reset
        }
    })
}

fn from_top(width: usize, height: usize) -> impl Iterator<Item = Action> {
    IterGenerator(move || {
        for x in 0..width {
            for y in 0..height {
                yield Action::Visit(x, y)
            }

            yield Action::Reset
        }
    })
}

fn from_bottom(width: usize, height: usize) -> impl Iterator<Item = Action> {
    IterGenerator(move || {
        for x in 0..width {
            for y in (0..height).rev() {
                yield Action::Visit(x, y)
            }

            yield Action::Reset
        }
    })
}

fn do_max_height<F, I>(arr: &Array2D, f: F) -> Array2D
where
    F: FnOnce(usize, usize) -> I,
    I: Iterator<Item = Action>,
{
    let mut res = Array2D::new(arr.width(), arr.height());
    let mut max_height = 0;

    for action in f(arr.width(), arr.height()) {
        match action {
            Action::Visit(x, y) => {
                res.set(x, y, max_height);
                max_height = cmp::max(max_height, arr.get(x, y));
            }
            Action::Reset => {
                max_height = 0;
            }
        }
    }

    res
}

fn do_max_dist<F, I>(arr: &Array2D, f: F) -> Array2D
where
    F: FnOnce(usize, usize) -> I,
    I: Iterator<Item = Action>,
{
    let mut res = Array2D::new(arr.width(), arr.height());
    let mut tree_dists = Vec::new();
    tree_dists.resize(10, 0);

    for action in f(arr.width(), arr.height()) {
        match action {
            Action::Visit(x, y) => {
                res.set(x, y, tree_dists[(arr.get(x, y) - 1) as usize]);

                for h in 0..10 {
                    if h <= arr.get(x, y) - 1 {
                        tree_dists[h as usize] = 1;
                    } else {
                        tree_dists[h as usize] += 1;
                    }
                }
            }
            Action::Reset => {
                for dist in &mut tree_dists {
                    *dist = 0;
                }
            }
        }
    }

    res
}

struct Array2D {
    data: Vec<u8>,
    width: usize,
}

impl fmt::Debug for Array2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height() {
            for x in 0..self.width() {
                write!(f, "{:02} ", self.get(x, y))?;
            }

            f.write_str("\n")?;
        }

        Ok(())
    }
}

impl Array2D {
    pub fn new(width: usize, height: usize) -> Self {
        let mut data = Vec::new();
        data.resize(width * height, 0);
        Self { data, width }
    }

    pub fn read_from(lines: impl Iterator<Item = String>) -> Result<Self, Error> {
        let mut data = Vec::new();
        let mut width = None;

        for line in lines {
            if let Some(width) = width {
                if line.len() != width {
                    return Err(Error::msg("Mismatched line length"));
                }
            } else {
                width = Some(line.len());
            }

            for ch in line.as_bytes() {
                let i = match ch {
                    b'0'..=b'9' => ch - b'0' + 1, // 0 becomes 1 so we can have 0 as a true minimum
                    _ => return Err(Error::msg("Character out of range")),
                };

                data.push(i);
            }
        }

        Ok(Self {
            data,
            width: width.unwrap_or(0),
        })
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.data.len() / self.width
    }

    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.data[x + y * self.width]
    }

    pub fn set(&mut self, x: usize, y: usize, v: u8) {
        self.data[x + y * self.width] = v;
    }
}

fn main() -> Result<(), Error> {
    let trees = Array2D::read_from(std::io::stdin().lines().map(|line| line.unwrap()))?;
    println!("trees\n{trees:?}");

    if PART1 {
        let from_left = do_max_height(&trees, from_left);
        let from_right = do_max_height(&trees, from_right);
        let from_top = do_max_height(&trees, from_top);
        let from_bottom = do_max_height(&trees, from_bottom);

        println!("from_left\n{from_left:?}");
        println!("from_right\n{from_right:?}");
        println!("from_top\n{from_top:?}");
        println!("from_bottom\n{from_bottom:?}");

        let mut visible = Array2D::new(trees.width(), trees.height());
        let mut num_visible = 0;

        for x in 0..trees.width() {
            for y in 0..trees.height() {
                let height = trees.get(x, y);
                if from_left.get(x, y) < height
                    || from_right.get(x, y) < height
                    || from_top.get(x, y) < height
                    || from_bottom.get(x, y) < height
                {
                    visible.set(x, y, 1);
                    num_visible += 1;
                }
            }
        }

        println!("visible\n{visible:?}");
        println!("num_visible = {num_visible}");
    } else {
        let from_left = do_max_dist(&trees, from_left);
        let from_right = do_max_dist(&trees, from_right);
        let from_top = do_max_dist(&trees, from_top);
        let from_bottom = do_max_dist(&trees, from_bottom);

        println!("from_left\n{from_left:?}");
        println!("from_right\n{from_right:?}");
        println!("from_top\n{from_top:?}");
        println!("from_bottom\n{from_bottom:?}");

        let mut best_pos_score = None;

        for x in 0..trees.width() {
            for y in 0..trees.height() {
                let score = from_left.get(x, y) as usize
                    * from_right.get(x, y) as usize
                    * from_top.get(x, y) as usize
                    * from_bottom.get(x, y) as usize;
                if let Some((best_pos, best_score)) = &mut best_pos_score {
                    if score > *best_score {
                        *best_pos = (x, y);
                        *best_score = score;
                    }
                } else {
                    best_pos_score = Some(((x, y), score));
                }
            }
        }

        println!("best_pos_score = {best_pos_score:?}");
    }

    Ok(())
}
