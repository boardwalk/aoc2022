use anyhow::Error;
use std::collections::HashSet;
use std::path::PathBuf;

const PART1: bool = false;

#[derive(Debug)]
struct Entry {
    path: PathBuf,
    size: u32,
}

fn main() -> Result<(), Error> {
    // parse the input
    let res: Result<Vec<_>, _> = std::io::stdin().lines().collect();
    let lines = res?;

    let mut cwd = PathBuf::from("/");
    let mut i = 0;
    let mut entries = Vec::new();

    while i < lines.len() {
        println!("{:?}", lines[i]);

        if let Some(dir) = lines[i].strip_prefix("$ cd ") {
            if dir.starts_with("/") {
                cwd = PathBuf::from(dir);
            } else if dir == ".." {
                cwd.pop();
            } else {
                cwd.push(dir);
            }

            i += 1;
        } else if lines[i] == "$ ls" {
            i += 1;

            while i < lines.len() && !lines[i].starts_with("$") {
                let tokens = lines[i].split_ascii_whitespace().collect::<Vec<_>>();

                if tokens[0] == "dir" {
                    // ignore
                } else {
                    let size = tokens[0].parse::<u32>()?;
                    let name = tokens[1];
                    entries.push(Entry {
                        path: cwd.join(name),
                        size,
                    });
                }

                i += 1;
            }
        } else {
            return Err(Error::msg("Unknown line prefix"));
        }
    }

    println!("{entries:?}");

    // collect all directories
    let mut dirs = HashSet::new();

    for entry in &entries {
        let mut path = entry.path.clone();
        while path.pop() {
            dirs.insert(path.clone());
        }
    }

    println!("{dirs:?}");

    // calculate sizes of directories
    let mut dir_sizes = Vec::new();

    for dir in &dirs {
        let size = entries
            .iter()
            .filter(|e| e.path.starts_with(dir))
            .map(|e| e.size)
            .sum::<u32>();

        dir_sizes.push(size);
    }

    dir_sizes.sort_unstable();

    println!("{dir_sizes:?}");

    let result = if PART1 {
        // calculate sum of directories with size <= 100000
        dir_sizes
            .into_iter()
            .take_while(|size| *size <= 100000)
            .sum::<u32>()
    } else {
        // calculate amount of space we need to free
        let min_to_free = dir_sizes.last().unwrap() - 40_000_000; // the root will always be the largest directory

        // find the size of the smallest directory to remove that will recover the needed space
        dir_sizes
            .into_iter()
            .filter(|size| *size >= min_to_free)
            .next()
            .unwrap()
    };

    println!("{result}");
    Ok(())
}
