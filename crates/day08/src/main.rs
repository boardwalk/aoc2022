use anyhow::Error;
use std::{cmp, fmt};

const PART1: bool = false;

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
    pub fn new<F>(width: usize, height: usize, f: F) -> Self
    where
        F: FnMut() -> u8,
    {
        let mut data = Vec::new();
        data.resize_with(width * height, f);
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
        // FROM_LEFT
        let mut from_left = Array2D::new(trees.width(), trees.height(), Default::default);

        for y in 0..trees.height() {
            let mut max_height = 0;
            for x in 0..trees.width() {
                from_left.set(x, y, max_height);
                max_height = cmp::max(max_height, trees.get(x, y));
            }
        }

        // FROM_RIGHT
        let mut from_right = Array2D::new(trees.width(), trees.height(), Default::default);

        for y in 0..trees.height() {
            let mut max_height = 0;
            for x in (0..trees.width()).rev() {
                from_right.set(x, y, max_height);
                max_height = cmp::max(max_height, trees.get(x, y));
            }
        }

        // FROM_TOP
        let mut from_top = Array2D::new(trees.width(), trees.height(), Default::default);

        for x in 0..trees.width() {
            let mut max_height = 0;
            for y in 0..trees.height() {
                from_top.set(x, y, max_height);
                max_height = cmp::max(max_height, trees.get(x, y));
            }
        }

        // FROM_BOTTOM
        let mut from_bottom = Array2D::new(trees.width(), trees.height(), Default::default);

        for x in 0..trees.width() {
            let mut max_height = 0;
            for y in (0..trees.height()).rev() {
                from_bottom.set(x, y, max_height);
                max_height = cmp::max(max_height, trees.get(x, y));
            }
        }

        println!("from_left\n{from_left:?}");
        println!("from_right\n{from_right:?}");
        println!("from_top\n{from_top:?}");
        println!("from_bottom\n{from_bottom:?}");

        let mut visible = Array2D::new(trees.width(), trees.height(), Default::default);
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
        // FROM_LEFT
        let mut from_left = Array2D::new(trees.width(), trees.height(), Default::default);

        for y in 0..trees.height() {
            let mut tree_dists = Vec::new();
            tree_dists.resize_with(10, || 0); // "at height i, you can see j trees to the left/right/top/bottom

            for x in 0..trees.width() {
                from_left.set(x, y, tree_dists[(trees.get(x, y) - 1) as usize]);

                for h in 0..10 {
                    if h <= trees.get(x, y) - 1 {
                        tree_dists[h as usize] = 1;
                    } else {
                        tree_dists[h as usize] += 1;
                    }
                }
            }
        }

        // FROM_RIGHT
        let mut from_right = Array2D::new(trees.width(), trees.height(), Default::default);

        for y in 0..trees.height() {
            let mut tree_dists = Vec::new();
            tree_dists.resize_with(10, || 0); // "at height i, you can see j trees to the left/right/top/bottom

            for x in (0..trees.width()).rev() {
                from_right.set(x, y, tree_dists[(trees.get(x, y) - 1) as usize]);

                for h in 0..10 {
                    if h <= trees.get(x, y) - 1 {
                        tree_dists[h as usize] = 1;
                    } else {
                        tree_dists[h as usize] += 1;
                    }
                }
            }
        }

        // FROM_TOP
        let mut from_top = Array2D::new(trees.width(), trees.height(), Default::default);

        for x in 0..trees.width() {
            let mut tree_dists = Vec::new();
            tree_dists.resize_with(10, || 0); // "at height i, you can see j trees to the left/right/top/bottom

            for y in 0..trees.height() {
                from_top.set(x, y, tree_dists[(trees.get(x, y) - 1) as usize]);

                for h in 0..10 {
                    if h <= trees.get(x, y) - 1 {
                        tree_dists[h as usize] = 1;
                    } else {
                        tree_dists[h as usize] += 1;
                    }
                }
            }
        }

        // FROM_BOTTOM
        let mut from_bottom = Array2D::new(trees.width(), trees.height(), Default::default);

        for x in 0..trees.width() {
            let mut tree_dists = Vec::new();
            tree_dists.resize_with(10, || 0); // "at height i, you can see j trees to the left/right/top/bottom

            for y in (0..trees.height()).rev() {
                from_bottom.set(x, y, tree_dists[(trees.get(x, y) - 1) as usize]);

                for h in 0..10 {
                    if h <= trees.get(x, y) - 1 {
                        tree_dists[h as usize] = 1;
                    } else {
                        tree_dists[h as usize] += 1;
                    }
                }
            }
        }

        let mut best_pos_score = None;

        for x in 0..trees.width() {
            for y in 0..trees.height() {
                let score = from_left.get(x, y) as usize
                    * from_right.get(x, y) as usize
                    * from_top.get(x, y) as usize
                    * from_bottom.get(x, y) as usize;
                if let Some((best_score, best_pos)) = &mut best_pos_score {
                    if score > *best_score {
                        *best_pos = (x, y);
                        *best_score = score;
                    }
                } else {
                    best_pos_score = Some((score, (x, y)));
                }
            }
        }

        println!("from_left\n{from_left:?}");
        println!("from_right\n{from_right:?}");
        println!("from_top\n{from_top:?}");
        println!("from_bottom\n{from_bottom:?}");
        println!("best_pos_score = {best_pos_score:?}");
    }

    Ok(())
}
