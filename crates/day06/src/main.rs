use anyhow::Error;
use std::collections::HashSet;

const PART1: bool = false;

fn all_different(chars: &[char]) -> bool {
    chars.into_iter().collect::<HashSet<_>>().len() == chars.len()
}

fn main() -> Result<(), Error> {
    let marker_len = if PART1 { 4 } else { 14 };

    for line in std::io::stdin().lines() {
        let line = line?;
        let line = line.chars().collect::<Vec<_>>();
        let marker_start =
            (marker_len..line.len()).find(|&i| all_different(&line[i - marker_len..i]));
        println!("{marker_start:?}");
    }

    Ok(())
}
