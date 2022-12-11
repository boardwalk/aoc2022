use anyhow::Error;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, i32};
use nom::multi::separated_list1;
use nom::IResult;
use std::collections::HashMap;
use std::io::Read;

#[derive(Debug)]
enum Operand {
    Var(String),
    Const(i32),
}

impl Operand {
    fn resolve(&self, vars: &HashMap<String, i32>) -> i32 {
        match self {
            Self::Var(name) => vars.get(name).copied().expect("Undefined variable"),
            Self::Const(value) => *value,
        }
    }
}

#[derive(Debug)]
enum Operator {
    Add,
    Mul,
}

impl Operator {
    fn execute(&self, left: i32, right: i32) -> i32 {
        match self {
            Self::Add => left + right,
            Self::Mul => left * right,
        }
    }
}

#[derive(Debug)]
struct Expr {
    left: Operand,
    right: Operand,
    oper: Operator,
}

impl Expr {
    fn eval(&self, vars: &HashMap<String, i32>) -> i32 {
        let left = self.left.resolve(vars);
        let right = self.right.resolve(vars);
        self.oper.execute(left, right)
    }
}

#[derive(Debug)]
struct Monkey {
    num: i32,
    initial_items: Vec<i32>,
    oper: Expr,
    test_divisor: i32,
    true_monkey: i32,
    false_monkey: i32,
}

#[derive(Debug)]
struct MonkeyState {
    items: Vec<i32>,
    inspect_count: i32,
}

fn operand_var(input: &str) -> IResult<&str, Operand> {
    let (input, ident) = alpha1(input)?;
    Ok((input, Operand::Var(ident.to_owned())))
}

fn operand_const(input: &str) -> IResult<&str, Operand> {
    let (input, value) = i32(input)?;
    Ok((input, Operand::Const(value)))
}

fn operand(input: &str) -> IResult<&str, Operand> {
    alt((operand_var, operand_const))(input)
}

fn operator_add(input: &str) -> IResult<&str, Operator> {
    let (input, _) = tag(" + ")(input)?;
    Ok((input, Operator::Add))
}

fn operator_mul(input: &str) -> IResult<&str, Operator> {
    let (input, _) = tag(" * ")(input)?;
    Ok((input, Operator::Mul))
}

fn operator(input: &str) -> IResult<&str, Operator> {
    alt((operator_add, operator_mul))(input)
}

fn expr(input: &str) -> IResult<&str, Expr> {
    let (input, left) = operand(input)?;
    let (input, oper) = operator(input)?;
    let (input, right) = operand(input)?;
    Ok((input, Expr { left, oper, right }))
}

fn monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _) = tag("Monkey ")(input)?;
    let (input, num) = i32(input)?;
    let (input, _) = tag(":\n  Starting items: ")(input)?;
    let (input, initial_items) = separated_list1(tag(", "), i32)(input)?;
    let (input, _) = tag("\n  Operation: new = ")(input)?;
    let (input, oper) = expr(input)?;
    let (input, _) = tag("\n  Test: divisible by ")(input)?;
    let (input, test_divisor) = i32(input)?;
    let (input, _) = tag("\n    If true: throw to monkey ")(input)?;
    let (input, true_monkey) = i32(input)?;
    let (input, _) = tag("\n    If false: throw to monkey ")(input)?;
    let (input, false_monkey) = i32(input)?;

    Ok((
        input,
        Monkey {
            num,
            initial_items,
            oper,
            test_divisor,
            true_monkey,
            false_monkey,
        },
    ))
}

fn main() -> Result<(), Error> {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf)?;

    let (_input, mut monkeys) = match separated_list1(tag("\n\n"), monkey)(&buf) {
        Ok(monkeys) => monkeys,
        Err(err) => {
            println!("{:?}", err);
            return Err(Error::msg("Parsing failed"));
        }
    };

    let mut states = monkeys
        .iter()
        .map(|monkey| MonkeyState {
            items: monkey.initial_items.clone(),
            inspect_count: 0,
        })
        .collect::<Vec<_>>();

    for _round in 0..20 {
        for (monkey_num, monkey) in monkeys.iter().enumerate() {
            let items = std::mem::take(&mut states[monkey_num].items);

            for mut item in items.into_iter() {
                // Update worry
                let mut vars = HashMap::new();
                vars.insert("old".to_owned(), item);
                item = monkey.oper.eval(&vars);

                // Update total inspection count
                states[monkey_num].inspect_count += 1;

                // Drop worry level
                item /= 3;

                // Pass item to next monkey
                let target_monkey = if item % monkey.test_divisor == 0 {
                    monkey.true_monkey
                } else {
                    monkey.false_monkey
                };

                states[target_monkey as usize].items.push(item);
            }
        }
    }

    let mut counts = states
        .iter()
        .map(|state| state.inspect_count)
        .collect::<Vec<_>>();

    counts.sort_unstable_by(|a, b| b.cmp(a));

    let monkey_business = counts.iter().take(2).fold(1, |acc, count| acc * count);

    println!("{:?}", monkeys);
    println!("{:?}", states);
    println!("{:?}", monkey_business);

    Ok(())
}
