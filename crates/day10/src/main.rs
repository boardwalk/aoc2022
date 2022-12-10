use anyhow::Error;

#[derive(Clone, Copy)]
enum Instr {
    Noop,
    Addx(i32),
}

impl Instr {
    fn cycles(self) -> u32 {
        match self {
            Self::Noop => 1,
            Self::Addx(_) => 2,
        }
    }

    fn execute(self, x: &mut i32) {
        match self {
            Self::Noop => (),
            Self::Addx(value) => {
                *x += value;
            }
        }
    }
}

fn main() -> Result<(), Error> {
    let mut instrs = Vec::new();

    for line in std::io::stdin().lines() {
        let line = line?;
        let instr = if line == "noop" {
            Instr::Noop
        } else if let Some(value) = line.strip_prefix("addx ") {
            let value = value.parse()?;
            Instr::Addx(value)
        } else {
            return Err(Error::msg("Bad instruction"));
        };

        instrs.push(instr);
    }

    let mut iter = instrs.iter().peekable();
    let mut instr_age = 0;

    let mut cycle = 0;
    let mut x = 1;
    let mut result = 0;
    let mut screen = Vec::new();
    screen.resize(40 * 6, false);

    while let Some(instr) = iter.peek() {
        if (cycle - 19) % 40 == 0 {
            result += (cycle + 1) * x;
        }

        if i32::abs_diff(x, cycle % 40) <= 1 {
            screen[cycle as usize] = true;
        }

        instr_age += 1;

        if instr_age == instr.cycles() {
            instr.execute(&mut x);
            iter.next();
            instr_age = 0;
        }

        cycle += 1;
    }

    println!("{result}");

    for y in 0..6 {
        let mut s = String::new();
        for x in 0..40 {
            let ch = if screen[y * 40 + x] { '#' } else { '.' };
            s.push(ch);
        }

        println!("{s}");
    }

    Ok(())
}
