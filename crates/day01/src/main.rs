use anyhow::Error;
use itertools::Itertools as _;

pub const PART1: bool = false;

fn main() -> Result<(), Error> {
    let mut elves = std::io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .group_by(|line| line.is_empty())
        .into_iter()
        .filter_map(|(is_empty, group)| if is_empty { None } else { Some(group) })
        .map(|group| group.map(|line| line.parse::<u32>().unwrap()).sum::<u32>())
        .collect::<Vec<_>>();

    elves.sort_unstable();

    let topn = if PART1 { 1 } else { 3 };
    let top_calories: u32 = elves.iter().rev().take(topn).sum();
    println!("{top_calories}");

    Ok(())
}
