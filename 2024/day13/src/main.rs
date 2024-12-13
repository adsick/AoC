use std::ops::Add;
use std::fmt::Debug;

const INPUT: &str = include_str!("../small.txt");

#[derive(Clone, Copy)]
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

#[derive(Debug)]
struct Task {
    a: Pos,
    b: Pos,
    d: Pos,
}

fn parse(input: &str) -> Task {
    let mut lines = input.lines();
    let mut ps = [Pos(0, 0); 3];
    for i in 0..3 {

        let op = if i < 2 { "+" } else { "=" };
        let del = (format!("X{op}"), format!("Y{op}"));

        let p: Pos = lines
        .next()
        .unwrap()
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

    Task { a, b, d }
}

fn main() {
    for t in INPUT.split("\n\n") {
        println!("{:?}", parse(t));
    }
}
