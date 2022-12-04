use std::ops::RangeInclusive;

const PART1: bool = false;

fn totally_includes(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
    b.start() >= a.start() && b.end() <= a.end()
}

fn overlaps(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
    !(a.end() < b.start() || a.start() > b.end())
}

fn main() {
    let count = std::io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let ranges = line
                .split(",")
                .map(|range| {
                    let hyphen_idx = range.find('-').unwrap();
                    let low = range[..hyphen_idx].parse::<u32>().unwrap();
                    let high = range[hyphen_idx + 1..].parse::<u32>().unwrap();
                    low..=high
                })
                .collect::<Vec<_>>();
            assert_eq!(ranges.len(), 2);
            ranges
        })
        .filter(|ranges| {
            if PART1 {
                totally_includes(&ranges[0], &ranges[1]) || totally_includes(&ranges[1], &ranges[0])
            } else {
                overlaps(&ranges[0], &ranges[1])
            }
        })
        .count();

    println!("{count}");
}
