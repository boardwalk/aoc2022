use anyhow::Error;
use std::collections::HashSet;

const PART1: bool = false;

fn all_different(chars: &[char]) -> bool {
    let mut set = HashSet::new();

    for ch in chars {
        if !set.insert(ch) {
            return false;
        }
    }

    true
}

fn main() -> Result<(), Error> {
    let marker_len = if PART1 { 4 } else { 14 };

    for line in std::io::stdin().lines() {
        let line = line?;
        let line = line.chars().collect::<Vec<_>>();
        let mut found = false;
        for i in marker_len..line.len() {
            let line_part = &line[i - marker_len..i];
            if all_different(line_part) {
                println!("starts at {i}");
                found = true;
                break;
            }
        }

        if !found {
            println!("not found");
        }
    }

    Ok(())
}
