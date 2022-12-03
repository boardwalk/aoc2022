use itertools::Itertools as _;

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

fn make_bit_set(line: &str) -> u64 {
    line.bytes().fold(0u64, |bits, b| bits | (1 << prio(b)))
}

fn sum_bits(mut x: u64) -> u32 {
    let mut base = 0;
    let mut sum = 0;

    while x > 0 {
        let tz = x.trailing_zeros();
        x >>= tz + 1;
        base += tz;
        sum += base;
    }

    sum
}

fn main() {
    if PART1 {
        let prio_sum = std::io::stdin()
            .lines()
            .map(|line| {
                let line = line.unwrap();
                assert_eq!(line.len() % 2, 0);
                let half_len = line.len() / 2;
                let left = make_bit_set(&line[..half_len]);
                let right = make_bit_set(&line[half_len..]);
                sum_bits(left & right)
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
                    .fold(None, |common_items: Option<u64>, line: String| {
                        let items = make_bit_set(&line);
                        if let Some(common_items) = common_items {
                            Some(common_items & items)
                        } else {
                            Some(items)
                        }
                    })
                    .unwrap();

                assert_ne!(badges, 0);
                sum_bits(badges)
            })
            .sum::<u32>();

        println!("{prio_sum}");
    }
}
