use regex::Regex;

const PART1: bool = false;

enum State {
    Stacks,
    Instructions,
}

struct Instruction {
    count: usize,
    stack_from: usize,
    stack_to: usize,
}

fn rev_string(s: &str) -> String {
    return s.chars().rev().collect::<String>();
}

fn main() {
    let crate_regex = Regex::new("\\[([A-Z])\\]").unwrap();
    let move_regex = Regex::new("move (\\d+) from (\\d+) to (\\d+)").unwrap();
    let mut state = State::Stacks;
    let mut stacks: Vec<String> = Vec::new();
    let mut instructions: Vec<Instruction> = Vec::new();

    for line in std::io::stdin().lines() {
        let line = line.unwrap();

        if line.is_empty() {
            state = State::Instructions;
            continue;
        }

        match state {
            State::Stacks => {
                let mut i = 0;
                while i * 4 < line.len() {
                    let token_len = std::cmp::min(line.len() - i * 4, 3);
                    let token = &line[i * 4..i * 4 + token_len].trim();

                    if !token.is_empty() {
                        let Some(captures) = crate_regex.captures(token) else {
                            break;
                        };

                        if i >= stacks.len() {
                            stacks.resize_with(i + 1, Default::default);
                        }

                        stacks[i].push_str(&captures[1]);
                    }

                    i += 1;
                }
            }
            State::Instructions => {
                let captures = move_regex.captures(&line).unwrap();
                let count = captures[1].parse().unwrap();
                let stack_from = captures[2].parse().unwrap();
                let stack_to = captures[3].parse().unwrap();

                instructions.push(Instruction {
                    count,
                    stack_from,
                    stack_to,
                });
            }
        }
    }

    for stack in &mut stacks {
        *stack = rev_string(stack);
    }

    for instr in &instructions {
        let mut to_move = String::new();

        for _i in 0..instr.count {
            let c = stacks[instr.stack_from - 1].pop().unwrap();
            to_move.push(c);
        }

        if !PART1 {
            to_move = rev_string(&to_move);
        }

        stacks[instr.stack_to - 1].push_str(&to_move);
    }

    let mut result = String::new();

    for stack in &stacks {
        if let Some(c) = stack.chars().last() {
            result.push(c);
        }
    }

    println!("{result}");
}
