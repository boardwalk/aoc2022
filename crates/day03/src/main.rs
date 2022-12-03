use itertools::Itertools as _;
use std::collections::HashSet;

const PART1: bool = false;

fn prio(b: u8) -> u32 {
    if b >= b'a' && b <= b'z' {
        (b - b'a' + 1) as u32
    } else if b >= b'A' && b <= b'Z' {
        (b - b'A' + 27) as u32
    } else {
        panic!("invalid byte");
    }
}

fn main() {
    if PART1 {
        let prio_sum = std::io::stdin()
            .lines()
            .map(|line| {
                let line = line.unwrap();
                assert_eq!(line.len() % 2, 0);
                let half_len = line.len() / 2;

                let left = line[..half_len].bytes().collect::<HashSet<_>>();
                let right = line[half_len..].bytes().collect::<HashSet<_>>();

                left.intersection(&right).map(|b| prio(*b)).sum::<u32>()
            })
            .sum::<u32>();

        println!("{prio_sum}");
    } else {
        let prio_sum = std::io::stdin()
            .lines()
            .map(|line| line.unwrap())
            .chunks(3)
            .into_iter()
            .map(|chunk| {
                let badges = chunk
                    .map(|line| line.bytes().collect::<HashSet<_>>())
                    .fold(None, |common_items: Option<HashSet<u8>>, items| {
                        if let Some(common_items) = common_items {
                            Some(
                                common_items
                                    .intersection(&items)
                                    .copied()
                                    .collect::<HashSet<_>>(),
                            )
                        } else {
                            Some(items)
                        }
                    })
                    .unwrap();

                assert_eq!(badges.len(), 1);
                badges.iter().map(|b| prio(*b)).sum::<u32>()
            })
            .sum::<u32>();

        println!("{prio_sum}");
    }
}
