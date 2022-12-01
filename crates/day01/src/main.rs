use anyhow::Error;

pub const PART1: bool = false;

#[derive(Debug, Default)]
struct Elf {
    calories: Vec<u32>,
}

fn main() -> Result<(), Error> {
    let mut elves: Vec<Elf> = Vec::new();
    let mut cur_elf = None;

    for line in std::io::stdin().lines() {
        let line = line?;

        if line.is_empty() {
            if let Some(elf) = cur_elf.take() {
                elves.push(elf);
            }
        } else {
            let elf = cur_elf.get_or_insert_with(Default::default);
            let calories: u32 = line.parse()?;
            elf.calories.push(calories);
        }
    }

    if let Some(elf) = cur_elf.take() {
        elves.push(elf);
    }

    // println!("{elves:?}");

    if PART1 {
        let top_calories: u32 = elves
            .iter()
            .map(|elf| elf.calories.iter().sum())
            .max()
            .unwrap();

        println!("{top_calories}");
    } else {
        let mut total_calories: Vec<u32> =
            elves.iter().map(|elf| elf.calories.iter().sum()).collect();
        total_calories.sort_unstable();
        let top3_calories: u32 = total_calories.iter().rev().take(3).sum();

        println!("{top3_calories}");
    }

    Ok(())
}
