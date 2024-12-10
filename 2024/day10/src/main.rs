use std::fmt::Display;

const INPUT: &str = include_str!("../input.txt");

fn in_bounds<T>(m: &Mat<T>, i: isize, j: isize) -> bool {
    i >= 0 && j >= 0 && i < m.len() as isize && j < m.len() as isize
}

fn printmap<T: Display>(m: &Mat<T>) {
    for r in m {
        for c in r {
            print!("{c:<3}");
        }
        println!();
    }
    println!();
}

fn printmapni(m: &Mat<Vec<Ni>>) {
    for r in m {
        for c in r {
            let c = c.len();
            print!("{c:<3}");
        }
        println!();
    }
    println!();
}

type Mat<T> = Vec<Vec<T>>;
type Mat3<T> = Vec<Mat<T>>;
type Ni = u32;

fn sumaround2(m: &Mat3<u32>, l: u8, i: usize, j: usize) -> u32 {
    let mut res = 0;

    for di in -1..=1 {
        for dj in -1..=1 {
            if di == 0 && dj == 0 {
                continue;
            }
            if di * dj != 0 {
                continue;
            }

            let i = i as isize + di;
            let j = j as isize + dj;
            if in_bounds(&m[l as usize], i, j) {
                res += m[l as usize][i as usize][j as usize]
            }
        }
    }

    res
}

fn sumaround1(m: &Mat3<Vec<Ni>>, l: u8, i: usize, j: usize) -> Vec<Ni> {
    let mut res = vec![];

    for di in -1..=1 {
        for dj in -1..=1 {
            if di == 0 && dj == 0 {
                continue;
            }
            if di * dj != 0 {
                continue;
            }

            let i = i as isize + di;
            let j = j as isize + dj;
            if in_bounds(&m[l as usize], i, j) {
                let nis = &m[l as usize][i as usize][j as usize];
                for n in nis {
                    if !res.contains(n) {
                        res.push(*n);
                    }
                }
            }
        }
    }

    res
}

fn ni<T>(m: &Mat<T>, i: usize, j: usize) -> Ni {
    (i * m.len() + j) as Ni
}

fn solve1(m: &Mat<u8>) -> u64 {
    let mut d = vec![vec![vec![vec![]; m.len()]; m.len()]; 10];

    for i in 0..m.len() {
        for j in 0..m.len() {
            if m[i][j] == 9 {
                d[9][i][j] = vec![ni(m, i, j)];
            }
        }
    }

    for l in (0..9).rev() {
        for i in 0..m.len() {
            for j in 0..m.len() {
                if m[i][j] == l {
                    d[l as usize][i][j] = sumaround1(&d, l + 1, i, j);
                }
            }
        }
    }

    for l in (0..=9).rev() {
        // println!("level: {l}");
        // printmapni(&d[l as usize]);
    }

    d[0].iter()
        .map(|r| r.iter().map(|n| n.len()).sum::<usize>() as u64)
        .sum()
}

fn solve2(m: &Mat<u8>) -> u64 {
    let mut d = vec![vec![vec![0; m.len()]; m.len()]; 10];

    for i in 0..m.len() {
        for j in 0..m.len() {
            if m[i][j] == 9 {
                d[9][i][j] = 1;
            }
        }
    }

    for l in (0..9).rev() {
        for i in 0..m.len() {
            for j in 0..m.len() {
                if m[i][j] == l {
                    d[l as usize][i][j] = sumaround2(&d, l + 1, i, j);
                }
            }
        }
    }

    for l in (0..=9).rev() {
        // println!("level: {l}");
        // printmap(&d[l as usize]);
    }

    d[0].iter()
        .map(|r| r.iter().map(|n| *n).sum::<u32>() as u64)
        .sum()
}

fn main() {
    let map: Mat<u8> = INPUT
        .lines()
        .map(|l| l.as_bytes().iter().map(|b| b - 48).collect())
        .collect();

    // printmap(&map);

    let ans1 = solve1(&map);
    let ans2 = solve2(&map);
    println!("ans1: {ans1}");
    println!("ans2: {ans2}");
}
