use anyhow::{anyhow, bail, ensure, Error};
use core::fmt;
use nom::branch::alt;
use nom::character::complete::{alpha1, char, i64, multispace0, newline, one_of, space0};
use nom::combinator::eof;
use nom::multi::many1;
use nom::IResult;
use std::collections::HashMap;
use std::fmt::Write as _;
use std::io::{read_to_string, stdin};
use std::ops::{Add, Div, Mul, Sub};

const PART1: bool = false;

type Prim = ruint::aliases::U4096;

fn gcd(a: Prim, b: Prim) -> Prim {
    if a == Prim::from(0) {
        b
    } else {
        gcd(b % a, a)
    }
}

fn lcm(a: Prim, b: Prim) -> Prim {
    (a * b) / gcd(a, b)
}

#[derive(Clone, Copy)]
struct Fraction {
    numer: Prim,
    denom: Prim,
}

impl fmt::Debug for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.denom != Prim::from(1) {
            write!(f, "{}/{}", self.numer, self.denom)
        } else {
            write!(f, "{}", self.numer)
        }
    }
}

impl Fraction {
    fn invert(self) -> Fraction {
        Self {
            numer: self.denom,
            denom: self.numer,
        }
    }

    fn reduce(self) -> Fraction {
        let divisor = gcd(self.numer, self.denom);
        Self {
            numer: self.numer / divisor,
            denom: self.denom / divisor,
        }
    }
}

impl Add for Fraction {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let denom = lcm(self.denom, other.denom);

        let numer_self = self.numer * (denom / self.denom);
        let numer_other = other.numer * (denom / other.denom);

        Self {
            numer: numer_self + numer_other,
            denom,
        }
        .reduce()
    }
}

impl Sub for Fraction {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let denom = lcm(self.denom, other.denom);

        let numer_self = self.numer * (denom / self.denom);
        let numer_other = other.numer * (denom / other.denom);

        Self {
            numer: numer_self - numer_other,
            denom,
        }
        .reduce()
    }
}

impl Mul for Fraction {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            numer: self.numer * other.numer,
            denom: self.denom * other.denom,
        }
        .reduce()
    }
}

impl Div for Fraction {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        self.mul(other.invert()).reduce()
    }
}

impl From<Prim> for Fraction {
    fn from(value: Prim) -> Self {
        Self {
            numer: value,
            denom: Prim::from(1),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl BinOp {
    fn eval(self, left: Fraction, right: Fraction, depth: u32) -> Fraction {
        let mut s = (0..depth).map(|_| "  ").collect::<String>();
        write!(&mut s, "{left:?} {self:?} {right:?}").unwrap();

        let res = match self {
            Self::Add => left.add(right),
            Self::Sub => left.sub(right),
            Self::Mul => left.mul(right),
            Self::Div => left.div(right),
        };

        write!(&mut s, " = {res:?}").unwrap();
        println!("{}", s);

        res
    }
}

#[derive(Debug)]
enum Expr<'a> {
    Const {
        value: Fraction,
    },
    BinOp {
        left: &'a str,
        right: &'a str,
        binop: BinOp,
    },
}

#[derive(Debug)]
struct Line<'a> {
    name: &'a str,
    expr: Expr<'a>,
}

fn parse_expr_const(mut input: &str) -> IResult<&str, Expr> {
    let value;
    (input, value) = i64(input)?;
    let value = Prim::from(value);
    let value = Fraction::from(value);
    Ok((input, Expr::Const { value }))
}

fn parse_expr_binop(mut input: &str) -> IResult<&str, Expr> {
    let (left, binop, right);
    (input, left) = alpha1(input)?;
    (input, _) = space0(input)?;
    (input, binop) = one_of("+-*/")(input)?;
    (input, _) = space0(input)?;
    (input, right) = alpha1(input)?;

    let binop = match binop {
        '+' => BinOp::Add,
        '-' => BinOp::Sub,
        '*' => BinOp::Mul,
        '/' => BinOp::Div,
        _ => unreachable!(),
    };

    Ok((input, Expr::BinOp { left, right, binop }))
}

fn parse_expr(input: &str) -> IResult<&str, Expr> {
    alt((parse_expr_const, parse_expr_binop))(input)
}

fn parse_line(mut input: &str) -> IResult<&str, Line> {
    let (name, expr);
    (input, name) = alpha1(input)?;
    (input, _) = char(':')(input)?;
    (input, _) = space0(input)?;
    (input, expr) = parse_expr(input)?;
    (input, _) = newline(input)?;
    let line = Line { name, expr };
    Ok((input, line))
}

fn parse_input(mut input: &str) -> IResult<&str, Vec<Line>> {
    let lines;
    (input, lines) = many1(parse_line)(input)?;
    (input, _) = multispace0(input)?;
    (input, _) = eof(input)?;
    Ok((input, lines))
}

fn resolve(name: &str, exprs_by_name: &HashMap<&str, Expr>, depth: u32) -> Result<Fraction, Error> {
    ensure!(name != "humn");

    let mut s = (0..depth).map(|_| "  ").collect::<String>();
    write!(&mut s, "eval({name})").unwrap();

    let expr = exprs_by_name
        .get(name)
        .ok_or_else(|| anyhow!("expr {name} not found"))?;

    let result = match expr {
        Expr::Const { value } => value.clone(),
        Expr::BinOp { left, right, binop } => {
            let left = resolve(left, exprs_by_name, depth + 1)?;
            let right = resolve(right, exprs_by_name, depth + 1)?;
            binop.eval(left, right, depth + 1)
        }
    };

    write!(&mut s, " = {result:?}").unwrap();
    println!("{s}");
    Ok(result)
}

fn expr_references(
    name: &str,
    search: &str,
    exprs_by_name: &HashMap<&str, Expr>,
) -> Result<bool, Error> {
    if name == search {
        return Ok(true);
    }

    let expr = exprs_by_name
        .get(name)
        .ok_or_else(|| anyhow!("expr {name} not found"))?;

    let (left, right) = match expr {
        Expr::Const { .. } => return Ok(false),
        Expr::BinOp { left, right, .. } => (left, right),
    };

    let res = if expr_references(left, search, exprs_by_name)? {
        true
    } else if expr_references(right, search, exprs_by_name)? {
        true
    } else {
        false
    };

    Ok(res)
}

fn converge(
    name: &str,
    target: Fraction,
    exprs_by_name: &HashMap<&str, Expr>,
    depth: u32,
) -> Result<Fraction, Error> {
    let mut s = (0..depth).map(|_| "  ").collect::<String>();
    write!(&mut s, "converge({name}, {target:?})").unwrap();

    let expr = exprs_by_name
        .get(name)
        .ok_or_else(|| anyhow!("expr {name} not found"))?;

    let res = match expr {
        Expr::Const { .. } => {
            // println!("converge const to {target}");
            ensure!(name == "humn");
            target
        }
        Expr::BinOp { left, right, binop } => {
            if expr_references(left, "humn", exprs_by_name)? {
                // left is free, right is fixed
                let right = resolve(right, exprs_by_name, depth + 1)?;

                let new_target = match binop {
                    BinOp::Add => target - right,
                    BinOp::Sub => target + right,
                    BinOp::Mul => target / right,
                    BinOp::Div => target * right,
                };

                converge(left, new_target, exprs_by_name, depth + 1)?
            } else if expr_references(right, "humn", exprs_by_name)? {
                // right is free, left is fixed
                let left = resolve(left, exprs_by_name, depth + 1)?;

                let new_target = match binop {
                    BinOp::Add => target - left,
                    BinOp::Sub => left - target,
                    BinOp::Mul => target / left,
                    BinOp::Div => left / target,
                };

                converge(right, new_target, exprs_by_name, depth + 1)?
            } else {
                bail!("humn not referenced by binop");
            }
        }
    };

    write!(&mut s, " = {res:?}").unwrap();
    println!("{s}");
    Ok(res)
}

fn main() -> Result<(), Error> {
    let input = read_to_string(stdin())?;
    let (_, lines) = parse_input(&input).map_err(|e| anyhow!("failed to parse input: {e:?}"))?;

    let mut exprs_by_name = HashMap::new();
    for line in lines.into_iter() {
        exprs_by_name.insert(line.name, line.expr);
    }

    // println!("{exprs_by_name:?}");

    if PART1 {
        let r = resolve("root", &exprs_by_name, 0)?;
        println!("{r:?}");
    } else {
        let expr = exprs_by_name
            .get("root")
            .ok_or_else(|| anyhow!("expr root not found"))?;

        let (left, right) = match expr {
            Expr::Const { .. } => bail!("root is a constant"),
            Expr::BinOp { left, right, .. } => (left, right),
        };

        let res = if expr_references(left, "humn", &exprs_by_name)? {
            // left is free, right is fixed
            let target = resolve(right, &exprs_by_name, 0)?;
            converge(left, target, &exprs_by_name, 0)?
        } else if expr_references(right, "humn", &exprs_by_name)? {
            // right is free, left is fixed
            let target = resolve(left, &exprs_by_name, 0)?;
            converge(right, target, &exprs_by_name, 0)?
        } else {
            bail!("humn not referenced by binop");
        };

        println!("{res:?}")
    }

    Ok(())
}
