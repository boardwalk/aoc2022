use std::ops::RangeInclusive;

const PART1: bool = false;

fn totally_includes(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
    b.start() >= a.start() && b.end() <= a.end()
}

fn overlaps(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
    // I think this can be simplified a bit
    let a_before_b = a.start() <= b.start() && a.end() >= b.start();
    let a_after_b = a.start() <= b.end() && a.end() >= b.end();
    let a_within_b = a.start() >= b.start() && a.end() <= b.end();
    let b_within_a = b.start() >= a.start() && b.end() <= a.end();
    a_before_b || a_after_b || a_within_b || b_within_a
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
