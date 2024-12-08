const INPUT: &str = include_str!("../input.txt");

fn solve(n: u64, ms: &[u64]) -> bool {
    // 2 -> 1
    // 3 -> 2 (4)
    // 4 -> 3 (8)
    println!("solving for {n}: {ms:?}");
    let combinations = (1u32 << ms.len() - 1) - 1;
    for mut i in 0..=combinations {
        // println!("")
        let mut t = ms[0];
        print!("{i:<12b}: {t}");
        for m in ms[1..].iter() {
            if i % 2 == 0 {
                print!(" + ");
                t += m;
            } else {
                print!(" * ");
                t *= m
            }
            print!("{m}");
            i /= 2;
        }
        println!();
        if t == n {
            return true;
        }
    }

    false
}
fn main() {
    let mut ans = 0;
    for l in INPUT.lines() {
        let Some((n, rest)) = l.split_once(':') else {
            break;
        };
        let n = n.parse().unwrap();
        let ms: Vec<_> = rest
            .trim_start()
            .split(' ')
            .map(|m| m.parse().unwrap())
            .collect();
        if solve(n, &ms) {
            println!("yes");
            ans += n;
        }
    }
    println!("{ans}");
}
