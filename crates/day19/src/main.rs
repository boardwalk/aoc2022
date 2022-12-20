use anyhow::Error;

const PART2: bool = false;

fn main() -> Result<(), Error> {
    let mut coords = std::io::stdin()
        .lines()
        .enumerate()
        .map(|(orig_idx, line)| {
            let line = line?;
            let coord: i64 = line.parse()?;
            Ok((orig_idx, coord))
        })
        .collect::<Result<Vec<_>, Error>>()?;

    let num_rounds = if PART2 {
        for (_, coord) in &mut coords {
            *coord *= 811589153;
        }

        10
    } else {
        1
    };

    for round_idx in 0..num_rounds {
        println!("round {round_idx}");

        for orig_idx in 0..coords.len() {
            let mut cur_idx = coords.iter().position(|(oi, _)| *oi == orig_idx).unwrap();
            let mut move_amt = coords[cur_idx].1;

            let next_idx = (cur_idx as i64)
                .wrapping_add(move_amt)
                .rem_euclid(coords.len() as i64 - 1) as usize;

            let tmp = coords.remove(cur_idx);
            coords.insert(next_idx, tmp);
        }
    }

    let zero_idx = coords.iter().position(|(_, coord)| *coord == 0).unwrap();
    let a = coords[(zero_idx + 1000) % coords.len()].1;
    let b = coords[(zero_idx + 2000) % coords.len()].1;
    let c = coords[(zero_idx + 3000) % coords.len()].1;
    let result = a + b + c;
    println!("{a} {b} {c} {result}");

    Ok(())
}
