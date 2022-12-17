#![feature(array_windows)]

use anyhow::{anyhow, Error};
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::character::complete::{char, i32, newline};
use nom::combinator::eof;
use nom::multi::separated_list1;
use nom::IResult;
use std::cmp::{max, min};
use std::fmt;
use std::io::Read;

const PART2: bool = true;

#[derive(Clone, Copy)]
pub struct Vector {
    dx: i32,
    dy: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    const MIN: Self = Self {
        x: i32::MIN,
        y: i32::MIN,
    };

    const MAX: Self = Self {
        x: i32::MAX,
        y: i32::MAX,
    };

    const SAND_DROP: Self = Self { x: 500, y: 0 };

    fn component_wise_min(self, other: Self) -> Self {
        Self {
            x: min(self.x, other.x),
            y: min(self.y, other.y),
        }
    }

    fn component_wise_max(self, other: Self) -> Self {
        Self {
            x: max(self.x, other.x),
            y: max(self.y, other.y),
        }
    }

    fn add(self, vector: Vector) -> Self {
        Self {
            x: self.x + vector.dx,
            y: self.y + vector.dy,
        }
    }

    fn sub(self, other: Self) -> Vector {
        Vector {
            dx: self.x - other.x,
            dy: self.y - other.y,
        }
    }
}

fn point(mut input: &str) -> IResult<&str, Point> {
    let (x, y);
    (input, x) = i32(input)?;
    (input, _) = char(',')(input)?;
    (input, y) = i32(input)?;
    Ok((input, Point { x, y }))
}

fn points(input: &str) -> IResult<&str, Vec<Point>> {
    separated_list1(tag(" -> "), point)(input)
}

fn parse(mut input: &str) -> IResult<&str, Vec<Vec<Point>>> {
    let lines;
    (input, lines) = separated_list1(newline, points)(input)?;
    (input, _) = multispace0(input)?;
    (input, _) = eof(input)?;
    Ok((input, lines))
}

#[derive(Clone, Copy, Debug)]
enum Cell {
    Air,
    Rock,
    Sand,
}

impl Cell {
    fn char(self) -> char {
        match self {
            Self::Air => '.',
            Self::Rock => '#',
            Self::Sand => 'o',
        }
    }

    fn is_air(self) -> bool {
        matches!(self, Self::Air)
    }
}

struct Cave {
    cells: Vec<Cell>,
    min_point: Point,
    max_point: Point,
}

impl fmt::Debug for Cave {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for y in self.min_point.y..=self.max_point.y {
            for x in self.min_point.x..=self.max_point.x {
                s.push(self.get(Point { x, y }).char());
            }

            s.push('\n');
        }

        f.write_str(&s)
    }
}

impl Cave {
    fn new(min_point: Point, max_point: Point) -> Self {
        let size = max_point.sub(min_point);

        let mut cells = Vec::new();
        cells.resize(((size.dx + 1) * (size.dy + 1)) as usize, Cell::Air);

        Self {
            cells,
            min_point,
            max_point,
        }
    }

    fn set(&mut self, point: Point, cell: Cell) {
        if point.x >= self.min_point.x
            && point.x <= self.max_point.x
            && point.y >= self.min_point.y
            && point.y <= self.max_point.y
        {
            let delta = point.sub(self.min_point);
            let index = delta.dx + delta.dy * (self.max_point.x - self.min_point.x + 1);
            self.cells[usize::try_from(index).unwrap()] = cell;
        } else {
            println!("Out of range set");
        }
    }

    fn get(&self, point: Point) -> Cell {
        if point.x >= self.min_point.x
            && point.x <= self.max_point.x
            && point.y >= self.min_point.y
            && point.y <= self.max_point.y
        {
            let delta = point.sub(self.min_point);
            let index = delta.dx + delta.dy * (self.max_point.x - self.min_point.x + 1);
            self.cells[usize::try_from(index).unwrap()]
        } else {
            Cell::Air
        }
    }
}

fn render_rock(cave: &mut Cave, mut a: Point, mut b: Point) {
    if a.x == b.x {
        let x = a.x;

        if a.y > b.y {
            std::mem::swap(&mut a, &mut b);
        }

        for y in a.y..=b.y {
            cave.set(Point { x, y }, Cell::Rock);
        }
    } else if a.y == b.y {
        let y = a.y;

        if a.x > b.x {
            std::mem::swap(&mut a, &mut b);
        }

        for x in a.x..=b.x {
            cave.set(Point { x, y }, Cell::Rock);
        }
    } else {
        panic!("Non-horizontal or vertical line");
    }
}

enum DropSandResult {
    Blocked,
    AtRest,
    IntoAbyss,
}

fn drop_sand(cave: &mut Cave) -> DropSandResult {
    let mut sand = Point::SAND_DROP;

    if !cave.get(sand).is_air() {
        return DropSandResult::Blocked;
    }

    loop {
        let deltas = [
            Vector { dx: 0, dy: 1 },
            Vector { dx: -1, dy: 1 },
            Vector { dx: 1, dy: 1 },
        ];

        let mut moved = false;

        for delta in &deltas {
            let new_sand = sand.add(*delta);
            if cave.get(new_sand).is_air() {
                sand = new_sand;
                moved = true;
                break;
            }
        }

        if moved {
            if sand.y > cave.max_point.y {
                return DropSandResult::IntoAbyss;
            }
        } else {
            cave.set(sand, Cell::Sand);
            return DropSandResult::AtRest;
        }
    }
}

fn calc_min_max(lines: &Vec<Vec<Point>>) -> (Point, Point) {
    let mut min_point = Point::MAX;
    let mut max_point = Point::MIN;

    for line in lines {
        for point in line {
            min_point = min_point.component_wise_min(*point);
            max_point = max_point.component_wise_max(*point);
        }
    }

    min_point = min_point.component_wise_min(Point::SAND_DROP);
    max_point = max_point.component_wise_max(Point::SAND_DROP);

    (min_point, max_point)
}

fn main() -> Result<(), Error> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    // println!("input: {input:?}");

    let (_, mut lines) = parse(&input).map_err(|e| anyhow!("Failed to parse: {e:?}"))?;
    // println!("lines: {lines:?}");

    let (mut min_point, mut max_point) = calc_min_max(&lines);

    if PART2 {
        let floor_begin = Point {
            x: min_point.x - 1000,
            y: max_point.y + 2,
        };

        let floor_end = Point {
            x: max_point.x + 1000,
            y: max_point.y + 2,
        };

        lines.push(vec![floor_begin, floor_end]);
        (min_point, max_point) = calc_min_max(&lines);
    }

    // println!("min = {min_point:?} max = {max_point:?}");

    let mut cave = Cave::new(min_point, max_point);

    for line in &lines {
        for [begin, end] in line.array_windows() {
            render_rock(&mut cave, *begin, *end);
        }
    }

    // println!("{cave:?}");

    let mut num_sand = 0;
    loop {
        match drop_sand(&mut cave) {
            DropSandResult::Blocked => {
                break;
            }
            DropSandResult::AtRest => (),
            DropSandResult::IntoAbyss => {
                if PART2 {
                    println!("Our infinite floor is not enough");
                }

                break;
            }
        }

        num_sand += 1;
    }

    // println!("{cave:?}");
    println!("num_sand = {num_sand}");

    Ok(())
}
