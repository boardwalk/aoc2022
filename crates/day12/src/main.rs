use anyhow::Error;
use std::fmt;

#[derive(Clone, Copy, Debug)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, Debug)]
struct Size {
    w: usize,
    h: usize,
}

impl Position {
    fn go(self, direction: Direction, size: Size) -> Option<Position> {
        match direction {
            Direction::Left => {
                if self.x > 0 {
                    Some(Self {
                        x: self.x - 1,
                        y: self.y,
                    })
                } else {
                    None
                }
            }
            Direction::Right => {
                if self.x < size.w - 1 {
                    Some(Self {
                        x: self.x + 1,
                        y: self.y,
                    })
                } else {
                    None
                }
            }
            Direction::Up => {
                if self.y > 0 {
                    Some(Self {
                        x: self.x,
                        y: self.y - 1,
                    })
                } else {
                    None
                }
            }
            Direction::Down => {
                if self.y < size.h - 1 {
                    Some(Self {
                        x: self.x,
                        y: self.y + 1,
                    })
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    const ALL: [Self; 4] = [Self::Left, Self::Right, Self::Up, Self::Down];
}

fn calc_pos(index: usize, width: usize) -> Position {
    let x = index % width;
    let y = index / width;
    Position { x, y }
}

struct HeightMap {
    data: Vec<u8>,
    width: usize,
    start_pos: Position,
    end_pos: Position,
}

impl fmt::Debug for HeightMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let size = self.size();
        for y in 0..size.h {
            for x in 0..size.w {
                write!(f, "{:02} ", self.get(Position { x, y }))?;
            }

            f.write_str("\n")?;
        }

        writeln!(f, "start_pos: {:?}", self.start_pos)?;
        writeln!(f, "end_pos: {:?}", self.end_pos)?;
        Ok(())
    }
}

impl HeightMap {
    pub fn read_from(lines: impl Iterator<Item = String>) -> Result<Self, Error> {
        let mut data = Vec::new();
        let mut width = None;
        let mut start_pos = None;
        let mut end_pos = None;

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
                    b'a'..=b'z' => ch - b'a',
                    b'S' => {
                        start_pos = Some(data.len());
                        0
                    }
                    b'E' => {
                        end_pos = Some(data.len());
                        26
                    }
                    _ => return Err(Error::msg("Character out of range")),
                };

                data.push(i);
            }
        }

        let width = width.ok_or_else(|| Error::msg("Missing width"))?;
        let start_pos = start_pos.ok_or_else(|| Error::msg("Missing start pos"))?;
        let end_pos = end_pos.ok_or_else(|| Error::msg("Missing end pos"))?;

        let start_pos = calc_pos(start_pos, width);
        let end_pos = calc_pos(end_pos, width);

        Ok(Self {
            data,
            width,
            start_pos,
            end_pos,
        })
    }

    pub fn size(&self) -> Size {
        Size {
            w: self.width,
            h: self.data.len() / self.width,
        }
    }

    pub fn start_pos(&self) -> Position {
        self.start_pos
    }

    pub fn end_pos(&self) -> Position {
        self.end_pos
    }

    pub fn get(&self, pos: Position) -> u8 {
        self.data[pos.x + pos.y * self.width]
    }
}

struct TempMap<T> {
    data: Vec<T>,
    width: usize,
}

impl<T> fmt::Debug for TempMap<T>
where
    T: fmt::Debug + Default + Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let size = self.size();
        for y in 0..size.h {
            for x in 0..size.w {
                write!(f, "{:?} ", self.get(Position { x, y }))?;
            }

            f.write_str("\n")?;
        }

        Ok(())
    }
}

impl<T> TempMap<T>
where
    T: Default + Copy,
{
    pub fn new(size: Size) -> Self {
        let mut data = Vec::new();
        data.resize_with(size.w * size.h, Default::default);
        Self {
            data,
            width: size.w,
        }
    }

    pub fn size(&self) -> Size {
        Size {
            w: self.width,
            h: self.data.len() / self.width,
        }
    }

    pub fn get(&self, pos: Position) -> T {
        self.data[pos.x + pos.y * self.width]
    }

    pub fn set(&mut self, pos: Position, v: T) {
        self.data[pos.x + pos.y * self.width] = v;
    }
}

#[derive(Clone, Copy, Debug)]
struct Breadcrumb {
    dist: u32,
}

struct WorkItem {
    position: Position, // current position
    dist: u32,          // distance from the end (the end itself has dist 0)
}

fn main() -> Result<(), Error> {
    let heights = HeightMap::read_from(std::io::stdin().lines().map(|line| line.unwrap()))?;
    println!("heights\n{heights:?}");

    let mut crumbs: TempMap<Option<Breadcrumb>> = TempMap::new(heights.size());

    let mut work_queue = vec![WorkItem {
        position: heights.end_pos(),
        dist: 0,
    }];

    while let Some(work) = work_queue.pop() {
        if let Some(crumb) = crumbs.get(work.position) {
            if crumb.dist <= work.dist {
                // existing crumb got here quicker
                continue;
            }

            // this crumb got here quicker
        } else {
            // no one has been here before!
        }

        // update crumb
        crumbs.set(work.position, Some(Breadcrumb { dist: work.dist }));

        // expand search in all directions
        for direction in Direction::ALL {
            if let Some(position) = work.position.go(direction, heights.size()) {
                let next_height = heights.get(work.position);
                let prev_height = heights.get(position);
                if next_height > prev_height + 1 {
                    continue;
                }

                work_queue.push(WorkItem {
                    position,
                    dist: work.dist + 1,
                });
            }
        }
    }

    println!("{crumbs:?}");

    let start_crumb = crumbs.get(heights.start_pos()).unwrap();
    println!("it took {}", start_crumb.dist);

    let mut best_dist = None;

    for x in 0..heights.size().w {
        for y in 0..heights.size().h {
            let Some(crumb) = crumbs.get(Position { x, y }) else {
                continue
            };

            if heights.get(Position { x, y }) != 0 {
                continue;
            }

            if let Some(d) = best_dist {
                if crumb.dist < d {
                    best_dist = Some(crumb.dist);
                }
            } else {
                best_dist = Some(crumb.dist);
            }
        }
    }

    println!("best hiking trail has dist {best_dist:?}");

    Ok(())
}
