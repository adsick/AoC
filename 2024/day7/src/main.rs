const INPUT: &str = include_str!("../input.txt");

fn concat(mut a: u64, mut b: u64) -> u64 {
    let bb = b;
    while b > 0 {
        b /= 10;
        a *= 10;
    }
    a + bb
}

fn powi(base: u64, p: u64) -> u64 {
    let mut res = 1;
    for _ in 0..p {
        res *= base;
    }
    res
}

fn solve1(n: u64, ms: &[u64]) -> bool {
    // println!("solving for {n}: {ms:?}");
    let combinations = (1u32 << ms.len() - 1) - 1;
    for mut i in 0..=combinations {
        let mut t = ms[0];
        // print!("{i:_>12b}: {t}");
        for m in ms[1..].iter() {
            if i % 2 == 0 {
                // print!(" + ");
                t += m;
            } else {
                // print!(" * ");
                t *= m
            }
            // print!("{m}");
            i /= 2;
        }
        // println!();
        if t == n {
            return true;
        }
    }

    false
}

fn solve2(n: u64, ms: &[u64]) -> bool {
    // println!("solving for {n}: {ms:?}");
    let combinations = powi(3, ms.len() as u64) - 1; // (1u32 << ms.len() - 1) - 1;
    for mut i in 0..=combinations {
        let mut t = ms[0];
        // print!("{i:_>12}: {t}");
        for m in ms[1..].iter() {
            match i % 3 {
                0 => {
                    // print!(" + ");
                    t += m;
                }
                1 => {
                    // print!(" * ");
                    t *= m
                }
                2 => {
                    // print!(" | ");
                    t = concat(t, *m);
                }
                _ => unreachable!(),
            }

            // print!("{m}");
            i /= 3;
        }
        if t == n {
            // println!(" = {t}");
            return true;
        } else {
            // println!(" = {t} != {n}");
        }
    }

    false
}

fn main() {
    let mut ans1 = 0;
    let mut ans2 = 0;

    for (ln, l) in INPUT.lines().enumerate() {
        let Some((n, rest)) = l.split_once(':') else {
            break;
        };
        println!("{ln}");

        let n = n.parse().unwrap();
        let ms: Vec<_> = rest
            .trim_start()
            .split(' ')
            .map(|m| m.parse().unwrap())
            .collect();
        if solve1(n, &ms) {
            println!("yes (1)");
            ans1 += n;
        }
        if solve2(n, &ms) {
            println!("yes (2)");
            ans2 += n;
        }
    }
    println!("ans1 = {ans1}\nans2 = {ans2}");
}
