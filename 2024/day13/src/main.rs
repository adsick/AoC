use std::fmt::Debug;
use aoc::{Pos, PosU32};

const INPUT: &str = include_str!("../input.txt");


#[derive(Debug, Clone, Copy)]
struct Task {
    a: PosU32,
    b: PosU32,
    d: PosU32,
}

fn parse(input: &str) -> Option<Task> {
    let mut lines = input.lines();
    let mut ps = [Pos::<u32>(0, 0); 3];
    for i in 0..3 {
        let op = if i < 2 { "+" } else { "=" };
        let del = (format!("X{op}"), format!("Y{op}"));

        let p: PosU32 = lines
            .next()?
            .split_once(&del.0)
            .unwrap()
            .1
            .split_once(", ")
            .map(|(x, y)| {
                (
                    x.parse().unwrap(),
                    y.split_once(&del.1).unwrap().1.parse().unwrap(),
                )
            })
            .unwrap()
            .into();
        ps[i] = p;
    }
    let a: PosU32 = ps[0];
    let b: PosU32 = ps[1];
    let d: PosU32 = ps[2];

    Some(Task { a, b, d })
}

fn solve1(t: Task) -> Option<u32> {
    let mut c = None;

    for b in (0..100).rev() {
        for a in (0..100).rev() {
            if t.a * a + t.b * b == t.d {
                c = Some(3 * a as u32 + b as u32)
            }
        }
    }

    c
}

fn main() {
    let mut ans1 = 0;
    for t in INPUT.split("\n\n") {
        let Some(t) = parse(t) else {
            break
        };
        let c = solve1(t);
        ans1 += c.unwrap_or_default();
        // println!("{:?} = {:?}", t, c);
    }
    println!("ans1: {ans1}");
}
