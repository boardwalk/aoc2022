use anyhow::anyhow;
use anyhow::Error;
use itertools::{EitherOrBoth, Itertools as _};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, i32, multispace0};
use nom::combinator::eof;
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::delimited;
use nom::IResult;
use std::cmp::Ordering;
use std::io::{stdin, Read as _};

pub const PART1: bool = false;

#[derive(Clone, Debug)]
pub enum Value {
    Integer(i32),
    List(Vec<Value>),
}

fn value_integer(input: &str) -> IResult<&str, Value> {
    let (input, i) = i32(input)?;
    Ok((input, Value::Integer(i)))
}

fn value_list(input: &str) -> IResult<&str, Value> {
    let (input, lst) = delimited(char('['), separated_list0(char(','), value), char(']'))(input)?;
    Ok((input, Value::List(lst)))
}

fn value(input: &str) -> IResult<&str, Value> {
    alt((value_integer, value_list))(input)
}

fn value_pair(input: &str) -> IResult<&str, (Value, Value)> {
    let (input, value1) = value(input)?;
    let (input, _) = char('\n')(input)?;
    let (input, value2) = value(input)?;
    Ok((input, (value1, value2)))
}

fn value_pairs(input: &str) -> IResult<&str, Vec<(Value, Value)>> {
    let (input, pairs) = separated_list1(tag("\n\n"), value_pair)(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = eof(input)?;
    Ok((input, pairs))
}

fn all_values(input: &str) -> IResult<&str, Vec<Value>> {
    separated_list0(multispace0, value)(input)
}

fn ordered(left: &Value, right: &Value) -> Ordering {
    // println!("Compare {left:?} vs {right:?}");
    match (left, right) {
        (Value::Integer(left), Value::Integer(right)) => Ord::cmp(left, right),
        (left @ Value::Integer(_), right) => {
            let left = Value::List(vec![left.clone()]);
            ordered(&left, right)
        }
        (left, right @ Value::Integer(_)) => {
            let right = Value::List(vec![right.clone()]);
            ordered(left, &right)
        }
        (Value::List(left), Value::List(right)) => {
            for pair in left.iter().zip_longest(right.iter()) {
                match pair {
                    // If both values are lists, compare the first value of each list, then the second value, and so on.
                    EitherOrBoth::Both(left, right) => {
                        let ord = ordered(left, right);
                        if ord != Ordering::Equal {
                            return ord;
                        }
                    }
                    // If the right list runs out of items first, the inputs are not in the right order.
                    EitherOrBoth::Left(_) => return Ordering::Greater,
                    // If the left list runs out of items first, the inputs are in the right order.
                    EitherOrBoth::Right(_) => return Ordering::Less,
                }
            }

            // If the lists are the same length and no comparison makes a decision about the order, continue checking the next part of the input.
            Ordering::Equal
        }
    }
}

fn main() -> Result<(), Error> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    if PART1 {
        let mut pairs = match value_pairs(&input) {
            Ok((_, pairs)) => pairs,
            Err(err) => return Err(anyhow!("Failed to parse input: {err:?}")),
        };

        // println!("{pairs:#?}");

        let mut sum = 0;

        for (i, (left, right)) in pairs.iter_mut().enumerate() {
            if ordered(left, right).is_le() {
                println!("{i} is ordered");
                sum += i + 1;
            } else {
                println!("{i} is not ordered");
            }
        }

        println!("sum: {sum}");
    } else {
        let mut values = match all_values(&input) {
            Ok((_, values)) => values,
            Err(err) => return Err(anyhow!("Failed to parse input: {err:?}")),
        };

        let divider1 = Value::List(vec![Value::List(vec![Value::Integer(2)])]);
        let divider2 = Value::List(vec![Value::List(vec![Value::Integer(6)])]);

        values.push(divider1.clone());
        values.push(divider2.clone());
        values.sort_by(|a, b| ordered(a, b));

        let index1 = values.binary_search_by(|v| ordered(v, &divider1)).unwrap();
        let index2 = values.binary_search_by(|v| ordered(v, &divider2)).unwrap();
        let decoder_key = (index1 + 1) * (index2 + 1);

        println!("decoder_key: {decoder_key}");
    }

    Ok(())
}
