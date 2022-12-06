use anyhow::Error;

const PART1: bool = false;

fn letter_index(ch: u8) -> u8 {
    match ch {
        b'a'..=b'z' => ch - b'a',
        b'A'..=b'Z' => ch - b'A' + 26,
        _ => panic!("invalid letter"),
    }
}

fn all_distinct(chars: &[u8]) -> bool {
    let num_distinct = chars
        .iter()
        .fold(0u64, |acc, ch| acc | (1 << letter_index(*ch)))
        .count_ones();
    num_distinct as usize == chars.len()
}

fn main() -> Result<(), Error> {
    let marker_len = if PART1 { 4 } else { 14 };

    for line in std::io::stdin().lines() {
        let line = line?;
        let line = line.as_bytes();
        let marker_start =
            (marker_len..line.len()).find(|&i| all_distinct(&line[i - marker_len..i]));
        println!("{marker_start:?}");
    }

    Ok(())
}
