use std::{
    fmt::{Debug, Display},
    fs::File,
    io::{BufRead as _, BufReader},
    num::ParseIntError,
    ops::{Add, Mul, Rem},
    str::FromStr,
};

pub fn read_input_rows<T: FromStr>() -> impl Iterator<Item = Vec<T>>
where
    T::Err: Debug,
{
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|row| parse_row(&row.unwrap()))
}

fn parse_row<T: FromStr>(line: &str) -> Vec<T>
where
    T::Err: Debug,
{
    line.split_whitespace()
        .map(|e| e.parse().unwrap())
        .collect()
}


pub type PosU32 = Pos<u32>;
pub type PosI32 = Pos<i32>;

pub struct Pos<T>(pub T, pub T);

impl<T> From<(T, T)> for Pos<T> {
    fn from(value: (T, T)) -> Self {
        Pos(value.0, value.1)
    }
}

impl<T> Add for Pos<T>
where
    T: Add<T, Output = T>,
{
    type Output = Pos<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Pos<T> {
    type Output = Pos<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Pos(self.0 * rhs, self.1 * rhs)
    }
}

impl<T: Rem<Output = T>> Rem for Pos<T> {
    type Output = Pos<T>;

    fn rem(self, rhs: Pos<T>) -> Self::Output {
        Pos(self.0 % rhs.0, self.1 % rhs.1)
    }
}

impl<T: Rem<Output = T> + Copy> Rem<T> for Pos<T> {
    type Output = Pos<T>;

    fn rem(self, rhs: T) -> Self::Output {
        Pos(self.0 % rhs, self.1 % rhs)
    }
}

impl<T> PartialEq for Pos<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl<T> Debug for Pos<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl<T> Display for Pos<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl<T> Clone for Pos<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        (self.0.clone(), self.1.clone()).into()
    }
}

impl<T: Copy> Copy for Pos<T> {}

#[derive(Debug)]
pub enum ParsePosError {
    MissingDelimiter,
    ParseIntErrorX(ParseIntError),
    ParseIntErrorY(ParseIntError),
}

impl Display for ParsePosError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsePosError::MissingDelimiter => write!(f, "missing delimiter (comma)"),
            ParsePosError::ParseIntErrorX(err) => write!(f, "Failed to parse X: {err}"),
            ParsePosError::ParseIntErrorY(err) => write!(f, "Failed to parse Y: {err}"),
        }
    }
}

impl<T: FromStr<Err = ParseIntError>> FromStr for Pos<T> {
    type Err = ParsePosError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or(ParsePosError::MissingDelimiter)?;
        Ok(Pos(
            x.parse().map_err(ParsePosError::ParseIntErrorX)?,
            y.parse().map_err(ParsePosError::ParseIntErrorY)?,
        ))
    }
}
