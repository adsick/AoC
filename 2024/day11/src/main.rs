use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");
const BLINK: usize = 75;

// I wanted to use recursion, but maybe I messed smth up and hit recursion depth limit (or size?)
// I ended up unwinding recursion to stack based approach which looks and feels more complicated.

type Stone = u64;
type Depth = usize; // recursion depth
type BackStone = u64; // back reference to the parent stone
type Count = u64; // count of "forks"
type Memo = Vec<HashMap<Stone, Count>>;
type Stack = Vec<(Depth, BackStone, Stone)>;

fn digs(mut n: u64) -> Vec<u64> {
    let mut res = vec![];

    while n > 0 {
        res.push(n % 10);
        n /= 10;
    }
    res
}

fn powi(n: u64, i: u64) -> u64 {
    let mut r = 1;
    for _ in 0..i {
        r *= n
    }
    r
}

fn split_st(mut ds: Vec<u64>) -> (u64, u64) {
    let mut a = 0;
    let mut b = 0;

    let mut ai = 0;
    let mut bi = 0;

    let l = ds.len();
    ds.reverse();
    let dsa = &ds[..l / 2];
    let dsb = &ds[l / 2..];

    for d in dsa.iter().rev() {
        a += d * powi(10, ai);
        ai += 1;
    }

    for d in &mut dsb.iter().rev() {
        b += d * powi(10, bi);
        bi += 1;
    }

    (a, b)
}

fn solve1(stones: &Vec<u64>) -> u64 {
    let mut next_stones = stones.clone();
    for _ in 0..25 {
        let prev_stones = next_stones;
        next_stones = Vec::with_capacity(prev_stones.len() * 2);

        for s in prev_stones.iter() {
            match s {
                0 => next_stones.push(1),
                1 => next_stones.push(2024),
                n => {
                    let ds = digs(*n);
                    if ds.len() % 2 == 0 {
                        let (a, b) = split_st(ds);
                        next_stones.push(a);
                        next_stones.push(b);
                    } else {
                        next_stones.push(*n * 2024);
                    }
                }
            }
        }
    }

    next_stones.len() as u64
}

#[allow(unused)]
fn print_cache(c: &Memo) {
    for (d, c) in c.iter().enumerate() {
        println!("{d:<2}: {c:?}");
    }
}

fn solve2(stack: &mut Stack, c: &mut Memo) {
    let Some((depth, back, stone)) = stack.last().copied() else {
        println!("empty stack");
        return;
    };

    if depth == BLINK {
        stack.pop();
        return;
    }

    if let Some(ans) = c[depth].get(&stone).copied() {
        // println!("cache hit ({depth}, {stone}) = {ans}");
        stack.pop();

        if depth > 0 {
            *c[depth - 1].entry(back).or_default() += ans;
            // println!("parent cache upd: ({}, {}) + {ans}", depth - 1, back);
        }

        return;
    }

    let depth = depth + 1;

    let mut f = 0;
    match stone {
        0 => stack.push((depth, stone, 1)),
        1 => stack.push((depth, stone, 2024)),
        n => {
            let ds = digs(n);

            if ds.len() % 2 == 0 {
                let (a, b) = split_st(ds);
                stack.push((depth, stone, a));
                stack.push((depth, stone, b));
                f = 1;
            } else {
                stack.push((depth, stone, n * 2024))
            }
        }
    };

    // "init" this cache entry
    *c[depth - 1].entry(stone).or_default() += f;
}

fn main() {
    let a: Vec<_> = INPUT
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let mut c: Memo = vec![Default::default(); BLINK];

    // let a = vec![125, 17];
    let mut ans2 = a.len() as u64;

    let mut stack: Vec<_> = a.iter().copied().map(|s| (0, 0, s)).collect();
    let mut max_stack = 0;
    let mut i: u64 = 0;

    while stack.len() > 0 {
        let stack_len = stack.len();
        println!("#{i:<6} stack: {:<12}", stack_len);
        solve2(&mut stack, &mut c);

        max_stack = max_stack.max(stack_len);
        i += 1;
        // println!();
    }
    println!();
    // print_cache(&c);

    let cs: usize = c.iter().map(|l| l.len()).sum();
    println!("cache size: {cs}");
    println!("max stack: {max_stack}");
    ans2 += a.iter().map(|s| c[0][s]).sum::<u64>();

    println!("ans1: {}", solve1(&a));
    println!("ans2: {}", ans2);
    assert_eq!(ans2, 205913561055242);
}
