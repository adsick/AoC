use std::{
    fmt::Debug,
    fs::File,
    io::{BufRead as _, BufReader},
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
