use std::fmt::Debug;
use std::ops::{Add, Mul};

const INPUT: &str = include_str!("../input.txt");

#[derive(Clone, Copy, Default)]
struct Pos(u32, u32);

impl From<(u32, u32)> for Pos {
    fn from(value: (u32, u32)) -> Self {
        Pos(value.0, value.1)
    }
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Mul<u32> for Pos {
    type Output = Pos;

    fn mul(self, rhs: u32) -> Self::Output {
        Pos(self.0 * rhs, self.1 * rhs)
    }
}

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug, Clone, Copy)]
struct Task {
    a: Pos,
    b: Pos,
    d: Pos,
}

fn parse(input: &str) -> Option<Task> {
    let mut lines = input.lines();
    let mut ps = [Pos(0, 0); 3];
    for i in 0..3 {
        let op = if i < 2 { "+" } else { "=" };
        let del = (format!("X{op}"), format!("Y{op}"));

        let p: Pos = lines
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
    let a: Pos = ps[0];
    let b: Pos = ps[1];
    let d: Pos = ps[2];

    Some(Task { a, b, d })
}

fn solve1(t: Task) -> Option<u32> {
    let mut c = None;

    for b in (0..100).rev() {
        for a in (0..100).rev() {
            if t.a * a + t.b * b == t.d {
                c = Some(3 * a + b)
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
