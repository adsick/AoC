use std::fs::read_to_string;

use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::sequence::separated_pair;
use nom::sequence::tuple;
use nom::IResult;

fn parse_do(input: &str) -> IResult<&str, ()> {
    let (input, _) = tag("do()")(input)?;
    Ok((input, ()))
}

fn parse_dont(input: &str) -> IResult<&str, ()> {
    let (input, _) = tag("don't()")(input)?;
    Ok((input, ()))
}

fn parse_num(input: &str) -> IResult<&str, u16> {
    map_res(digit1, str::parse)(input)
}

fn parse_mul(input: &str) -> IResult<&str, (u16, u16)> {
    let (i, (_, (a, b), _)) = tuple((
        tag("mul("),
        separated_pair(parse_num, char(','), parse_num),
        char(')'),
    ))(input)?;
    Ok((i, (a, b)))
}

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let mut ans: u32 = 0;

    let mut i = 0;
    let mut e = true;

    while i < input.len() {
        let s = &input[i..];
        i += 1;

        if parse_do(s).is_ok() {
            e = true
        } else if parse_dont(s).is_ok() {
            e = false
        } else if e {
            if let Ok((_, (a, b))) = parse_mul(s) {
                ans += a as u32 * b as u32;
            }
        }
    }

    println!("{ans}")
}
