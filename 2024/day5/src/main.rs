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
    let mut incorrect = vec![];

    for line in lines {
        let mut met = vec![];

        let numbers: Vec<u8> = line.split(',').map(|n| n.parse().unwrap()).collect();

        'l: for n in &numbers {
            if let Some(forbidden) = rules.get(&n) {
                for m in &met {
                    if forbidden.contains(m) {
                        met.clear();
                        break 'l;
                    }
                }
            }

            met.push(*n);
        }

        met.get(met.len() / 2)
            .map(|v| sum += *v as u32)
            .or_else(|| Some(incorrect.push(numbers)));
    }

    println!("ans1: {sum}\n");

    // bubble sort kind of a solution
    for l in &mut incorrect {
        let mut flag = true;
        while flag {
            flag = false;
            'i: for i in 1..l.len() {
                for j in 0..i {
                    let a = l[i];
                    let b = l[j];

                    if let Some(forbidden) = rules.get(&a) {
                        if forbidden.contains(&b) {
                            print!("{l:?} -> ");
                            l.swap(i, j);
                            println!("{l:?} (swapped {} with {})", l[j], l[i]);
                            flag = true;
                            break 'i;
                        }
                    }
                }
            }
        }
    }

    let ans2: u32 = incorrect
        .iter()
        .map(|l| *l.get(l.len() / 2).unwrap() as u32)
        .sum();
    println!("ans2: {ans2}")
}

// 0  1  2  3  4     0  1  2  3  4
// 75,47,61,53,29 -> 97,75,47,61,53
// 61,13,29       -> 61,29,13
// 97,13,75,29,47 -> 97,75,47,29,13
