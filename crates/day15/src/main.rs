use anyhow::Error;
use std::ops::Range;

const PART1: bool = false;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Sensor {
    sensor: Point,
    beacon: Point,
}

#[derive(Debug, Default)]
struct Exclusions {
    ranges: Vec<Range<i32>>,
}

impl Exclusions {
    fn insert(&mut self, exclusion: Range<i32>) {
        let idx = match self
            .ranges
            .binary_search_by_key(&exclusion.start, |range| range.start)
        {
            Ok(idx) => idx,
            Err(idx) => idx,
        };

        self.ranges.insert(idx, exclusion);

        if idx > 0 {
            self.try_merge_ranges(idx - 1);
        }

        self.try_merge_ranges(idx);
    }

    fn try_merge_ranges(&mut self, idx: usize) {
        loop {
            let Some(next_range) = self.ranges.get(idx + 1).cloned() else { break };
            let Some(this_range) = self.ranges.get_mut(idx) else { break };

            if this_range.end < next_range.start {
                break;
            }

            this_range.end = std::cmp::max(this_range.end, next_range.end);
            self.ranges.remove(idx + 1);
        }
    }
}

fn calc_exclusions_for_y(sensors: &[Sensor], distress_y: i32, exclusions: &mut Exclusions) {
    exclusions.ranges.clear();

    for sensor in sensors {
        // println!("{sensor:?}");

        let dist_to_beacon = (i32::abs_diff(sensor.sensor.x, sensor.beacon.x)
            + i32::abs_diff(sensor.sensor.y, sensor.beacon.y)) as i32;
        // println!("dist_to_beacon: {dist_to_beacon:?}");

        let dist_to_distress_y = (i32::abs_diff(sensor.sensor.y, distress_y)) as i32;
        // println!("dist_to_distress_y: {dist_to_distress_y:?}");

        if dist_to_beacon < dist_to_distress_y {
            continue;
        }

        let exclusion_count = dist_to_beacon - dist_to_distress_y;
        // println!("exclusion_count: {exclusion_count}");

        let exclusion = sensor.sensor.x - exclusion_count..sensor.sensor.x + exclusion_count + 1;
        exclusions.insert(exclusion);
    }
}

fn make_inverse(ranges: &[Range<i32>], inverse_ranges: &mut Vec<Range<i32>>) {
    inverse_ranges.clear();

    let mut prev_end = i32::MIN;

    for range in ranges {
        if prev_end < range.start {
            inverse_ranges.push(prev_end..range.start);
            prev_end = range.end;
        }
    }

    if prev_end < i32::MAX {
        inverse_ranges.push(prev_end..i32::MAX);
    }
}

fn overlaps(a: &Range<i32>, b: &Range<i32>) -> bool {
    !(a.end <= b.start || a.start >= b.end)
}

fn main() -> Result<(), Error> {
    let re = regex::Regex::new(
        r#"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)"#,
    )
    .unwrap();

    let sensors = std::io::stdin()
        .lines()
        .map(|line| {
            let line = line?;
            let captures = re
                .captures(&line)
                .ok_or_else(|| Error::msg("Line did not match regex"))?;
            let sx = captures[1].parse()?;
            let sy = captures[2].parse()?;
            let bx = captures[3].parse()?;
            let by = captures[4].parse()?;
            Ok(Sensor {
                sensor: Point { x: sx, y: sy },
                beacon: Point { x: bx, y: by },
            })
        })
        .collect::<Result<Vec<_>, Error>>()?;

    if PART1 {
        let distress_y: i32 = std::env::args()
            .skip(1)
            .next()
            .ok_or_else(|| Error::msg("Missing distress_y argument"))?
            .parse()?;

        let mut exclusions = Exclusions::default();
        calc_exclusions_for_y(&sensors, distress_y, &mut exclusions);
        println!("{exclusions:?}");

        let sum = exclusions
            .ranges
            .iter()
            .map(|range| range.end - range.start)
            .sum::<i32>();
        println!("{sum}");
    } else {
        let max_val: i32 = std::env::args()
            .skip(1)
            .next()
            .ok_or_else(|| Error::msg("Missing max_val argument"))?
            .parse()?;

        let mut exclusions = Exclusions::default();
        let mut inclusions = Vec::new();

        for distress_y in 0..=max_val {
            calc_exclusions_for_y(&sensors, distress_y, &mut exclusions);
            // println!("{exclusions:?}");
            make_inverse(&exclusions.ranges, &mut inclusions);
            // println!("{inclusions:?}");

            if let Some(distress_x) = inclusions
                .iter()
                .find(|range| overlaps(range, &(0..max_val + 1)))
            {
                assert!(distress_x.end == distress_x.start + 1);
                println!("found: {},{}", distress_x.start, distress_y);
                println!(
                    "frequency: {}",
                    distress_x.start as u64 * 4000000 + distress_y as u64
                );
                break;
            }
        }
    }

    Ok(())
}
