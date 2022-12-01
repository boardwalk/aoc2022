use anyhow::Error;
use itertools::Itertools as _;

pub const PART1: bool = false;

#[derive(Debug, Default)]
struct Elf {
    calories: Vec<u32>,
}

fn main() -> Result<(), Error> {
    let elves = std::io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .group_by(|line| line.is_empty())
        .into_iter()
        .filter_map(|(is_empty, group)| if is_empty { None } else { Some(group) })
        .map(|group| {
            let calories = group.map(|line| line.parse().unwrap()).collect();
            Elf { calories }
        })
        .collect::<Vec<_>>();

    // println!("{elves:?}");

    let mut total_calories: Vec<_> = elves.iter().map(|elf| elf.calories.iter().sum()).collect();
    total_calories.sort_unstable();

    if PART1 {
        let top_calories: u32 = total_calories.iter().rev().take(1).sum();
        println!("{top_calories}");
    } else {
        let top3_calories: u32 = total_calories.iter().rev().take(3).sum();
        println!("{top3_calories}");
    }

    Ok(())
}
