use crate::Day;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Result as IoResult;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{complete, map},
    multi::fold_many0,
    sequence::tuple,
    IResult,
};

#[derive(Debug, Copy, Clone)]
pub enum BinOp {
    Add,
    Mul,
    Sub,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Bin(Box<Expr>, BinOp, Box<Expr>),
    Unary(i64),
}

impl Expr {
    fn run(&self) -> i64 {
        match self {
            Expr::Bin(b1, op, b2) => {
                let b1 = b1.run();
                let b2 = b2.run();
                match op {
                    BinOp::Add => b1 + b2,
                    BinOp::Sub => b1 - b2,
                    BinOp::Mul => b1 * b2,
                }
            }
            Expr::Unary(n) => *n,
        }
    }
}

fn number(input: &str) -> IResult<&str, Expr> {
    map(digit1, |digits: &str| {
        Expr::Unary(digits.parse::<i64>().unwrap())
    })(input)
}

fn paren_expr<F>(input: &str, expr: F) -> IResult<&str, Expr>
where
    F: FnMut(&str) -> IResult<&str, Expr>,
{
    map(tuple((tag("("), expr, tag(")"))), |(_, b, _)| b)(input)
}

fn numop(input: &str) -> IResult<&str, BinOp> {
    map(alt((tag("+"), tag("-"), tag("*"))), |c: &str| {
        match c.chars().next().unwrap() {
            '+' => BinOp::Add,
            '-' => BinOp::Sub,
            '*' => BinOp::Mul,
            _ => panic!(),
        }
    })(input)
}

fn high_numop(input: &str) -> IResult<&str, BinOp> {
    map(alt((tag("+"), tag("-"))), |c: &str| {
        match c.chars().next().unwrap() {
            '+' => BinOp::Add,
            '-' => BinOp::Sub,
            '*' => BinOp::Mul,
            _ => panic!(),
        }
    })(input)
}

fn low_numop(input: &str) -> IResult<&str, BinOp> {
    map(tag("*"), |c: &str| match c.chars().next().unwrap() {
        '+' => BinOp::Add,
        '-' => BinOp::Sub,
        '*' => BinOp::Mul,
        _ => panic!(),
    })(input)
}

fn noprec_expr(input: &str) -> IResult<&str, Expr> {
    let (input, start) = alt((number, |s| paren_expr(s, noprec_expr)))(input)?;

    fold_many0(
        tuple((numop, alt((number, |s| paren_expr(s, noprec_expr))))),
        move || start.clone(),
        |state, (op, e)| Expr::Bin(Box::new(state), op, Box::new(e)),
    )(input)
}

fn term(input: &str) -> IResult<&str, Expr> {
    let (input, start) = alt((number, |s| paren_expr(s, prec_expr)))(input)?;

    fold_many0(
        tuple((high_numop, alt((number, |s| paren_expr(s, prec_expr))))),
        move || start.clone(),
        |state, (op, e)| Expr::Bin(Box::new(state), op, Box::new(e)),
    )(input)
}

fn prec_expr(input: &str) -> IResult<&str, Expr> {
    let (input, start) = term(input)?;

    fold_many0(
        tuple((low_numop, alt((term, |s| paren_expr(s, prec_expr))))),
        move || start.clone(),
        |state, (op, e)| Expr::Bin(Box::new(state), op, Box::new(e)),
    )(input)
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 18;
    type Input = String;
    type Sol1 = i64;
    type Sol2 = i64;

    fn process_input<R>(r: R) -> IoResult<Vec<Self::Input>>
    where
        R: std::io::BufRead,
    {
        r.lines()
            .map(|l| {
                let l = l?;
                Ok(l.chars().filter(|&x| x != ' ').collect::<String>())
            })
            .collect()
    }
    fn p1(v: &[Self::Input]) -> Self::Sol1 {
        v.iter()
            .map(|l| complete(noprec_expr)(l).unwrap().1)
            .map(|e| e.run())
            .sum()
    }
    fn p2(v: &[Self::Input]) -> Self::Sol2 {
        v.iter()
            .map(|l| complete(prec_expr)(l).unwrap().1)
            .map(|e| e.run())
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let s = "1 + 2 * 3 + 4 * 5 + 6";
        let v = Solution::process_input(std::io::BufReader::new(s.as_bytes())).unwrap();
        assert_eq!(Solution::p1(&v), 71);
        assert_eq!(Solution::p2(&v), 231);
    }
}
