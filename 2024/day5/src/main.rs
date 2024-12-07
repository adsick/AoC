use std::collections::{HashMap, HashSet};

pub const INPUT: &str = include_str!("../input.txt");

type Rules = HashMap<u8, HashSet<u8>>;

fn main() {
    let mut rules = Rules::default();

    let mut lines = INPUT.lines().into_iter();

    for line in &mut lines {
        let Some((left, right)) = line.split_once('|') else {
            break;
        };

        let left = left.parse().unwrap();
        let right = right.parse().unwrap();

        let forbidden = rules.entry(left).or_default();

        forbidden.insert(right);
    }

    let mut sum: u32 = 0;

    for line in lines {
        let mut met = vec![];

        'l: for n in line.split(',') {
            let n: u8 = n.parse().unwrap();

            if let Some(forbidden) = rules.get(&n) {
                for m in &met {
                    if forbidden.contains(m) {
                        met.clear();
                        break 'l;
                    }
                }
            }

            met.push(n);
        }

        met.get(met.len() / 2).map(|v| sum += *v as u32);
    }

    println!("ans: {sum}");
}
