use std::{collections::HashMap, fs::File, io::Read};

fn read_input() -> (Vec<u32>, Vec<u32>) {
    let mut file = File::open("input.txt").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    let (mut t1, mut t2) = (Vec::new(), Vec::new());
    buf.lines()
        .flat_map(|l| l.split_once("   "))
        .for_each(|(s1, s2)| {
            t1.push(s1.parse().unwrap());
            t2.push(s2.parse().unwrap());
        });
    (t1, t2)
}

fn main() {
    let (mut t1, mut t2) = read_input();

    println!("{} {}", t1.len(), t2.len());

    t1.sort();
    t2.sort();

    let ans1: u64 = t1
        .iter()
        .zip(t2.iter())
        .map(|(e1, e2)| e1.abs_diff(*e2) as u64)
        .sum();

    println!("ans1: {ans1}");

    let mut f: HashMap<u32, u32> = HashMap::new();

    t2.iter().for_each(|&e| {
        f.entry(e).or_insert_with(|| {
            t2.iter()
                .fold(0, |acc, el| if el == &e { acc + 1 } else { acc })
        });
    });

    let ans2: u32 = t1.iter().flat_map(|e| f.get(e).map(|f| f * e)).sum();

    println!("ans2: {ans2}")
}
