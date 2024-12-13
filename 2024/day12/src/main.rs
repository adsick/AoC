use std::collections::{BTreeSet, HashMap};

use colored::{control::set_override, Color, Colorize, CustomColor};
use rand::{Rng, SeedableRng};

const TINY: &str = include_str!("../tiny.txt");
const NESTED: &str = include_str!("../nested.txt");
const SMALL: &str = include_str!("../small.txt");
const INPUT: &str = include_str!("../input.txt");

// 140 x 140

type R = u32;
type C = u32;
type A = u32;
type P = u32;
// type Mat = Vec<Vec<N>>;
type Mat = Vec<Vec<A>>;
type Regions = HashMap<R, (A, P)>;

#[derive(Debug)]
struct Cluster(BTreeSet<C>);

impl Cluster {
    pub fn new() -> Self {
        Self(BTreeSet::new())
    }

    pub fn new2(pair: (C, C)) -> Self {
        let mut set = BTreeSet::new();
        set.insert(pair.0);
        set.insert(pair.1);
        Self(set)
    }

    pub fn insert(&mut self, el: C) {
        self.0.insert(el);
    }

    pub fn insert2(&mut self, pair: (C, C)) {
        self.0.insert(pair.0);
        self.0.insert(pair.1);
    }
}

impl PartialEq for Cluster {
    fn eq(&self, other: &Self) -> bool {
        self.0.intersection(&other.0).next().is_some()
    }
}

impl PartialEq<(C, C)> for Cluster {
    fn eq(&self, other: &(C, C)) -> bool {
        self.0.contains(&other.0) || self.0.contains(&other.1)
    }
}

#[derive(Debug)]
pub struct Clusters(Vec<Cluster>);

impl Clusters {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn insert2(&mut self, pair: (C, C)) {
        // let mut clusters: Vec<(usize, &Cluster)> = self.0.iter().enumerate().filter(|(i, cl)|*cl == &pair).collect();
        let mut new_cluster = Cluster::new();
        self.0.retain(|cl|{
            if cl == &pair {
                new_cluster.0.extend(&cl.0);

                false
            } else {
                true
            }
        });

        self.0.push(new_cluster);



        // if clusters.len() > 1 {
        //     eprintln!("found {} appropriate clusters", clusters.len());
        //     let main = &mut clusters[0];
        //     let mut removed = 0;
        //     for (i, cs) in clusters {

        //     }
        // }

        // if let Some(cluster) = self.0.iter_mut().find(|cl| *cl == &pair) {
        //     cluster.insert2(pair);
        // } else {
        //     self.0.push(Cluster::new2(pair));
        // }
    }

    pub fn insert(&mut self, el: C) -> bool {
        if self.0.iter_mut().find(|cl| cl.0.contains(&el)).is_none() {
            let mut cl = Cluster::new();
            cl.insert(el);
            self.0.push(cl);
            true
        } else {
            false
        }
    }

    pub fn find(&self, el: C) -> usize {
        self.0
            .iter()
            .enumerate()
            .find(|(_, cl)| cl.0.contains(&el))
            .map(|(i, _)| i + 1)
            .unwrap_or_default()
    }
}

fn solve1(m: &Mat) -> u64 {
    // let mut m = m.clone();
    let s = m.len();
    let mut mask: Mat = vec![vec![0; s]; s];
    let mut clusters = Clusters::new();
    let mut regions = Regions::new();

    // 1st pass
    let mut r = 0;
    for i in 0..s {
        let mut p = 0;
        for j in 0..s {
            let c = m[i][j];
            let cm = if c == p {
                mask[i][j - 1]
            } else if c == get_or0(&m, i, j, -1, 0) {
                mask[i - 1][j]
            } else {
                r += 1;
                r
            };

            mask[i][j] = cm;

            clusters.insert(cm);

            p = c;
        }
    }

    // printm(&mask);

    // 2nd pass
    for i in 0..s {
        let mut p = 0;
        for j in 0..s {
            let c = m[i][j];
            let cm = mask[i][j];

            let mut pm_h = 0;
            if c == p {
                pm_h = get_or0(&mask, i, j, 0, -1)
            }

            if pm_h == cm {
                pm_h = 0;
            }

            let mut pm_v = 0;
            if c == get_or0(&m, i, j, -1, 0) {
                pm_v = get_or0(&mask, i, j, -1, 0);
            }

            if pm_v == cm {
                pm_v = 0;
            }

            if c == 21 && get_or0(&mask, i, j, -1, 0) == 92 {
                eprintln!("here: {pm_v}, {pm_h}")
            }

            if pm_h > 0 {
                let pair = (pm_h as C, cm as C);
                clusters.insert2(pair);
            }

            if pm_v > 0 {
                let pair = (pm_v as C, cm as C);
                clusters.insert2(pair);
            }

            p = c;
        }
    }

    // 3rd pass
    for i in 0..s {
        for j in 0..s {
            let cm = mask[i][j];
            let r = clusters.find(cm);

            let e = regions.entry(r as u32).or_default();
            let a = &mut e.0;
            let p = &mut e.1;

            *a += 1;

            // idea: store "sides" as something like "origin" * "direction" in a set for each region
            for (di, dj) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
                let nm = get_or0(&mask, i, j, di, dj);
                if nm == 0 {
                    // nonexistent neighbors increase perimeter too (map borders)
                    *p += 1;
                    continue;
                }

                let nr = clusters.find(nm);
                if nr != r {
                    // todo: insert side
                    *p += 1;
                }
            }
        }
    }

    set_override(true);

    let random_color = |r: u32| {
        if r == 0 {
            return CustomColor::new(0, 0, 0,);
        }
        let mut rng = rand::prelude::StdRng::seed_from_u64(r as u64);
        let r = rng.gen_range(0..127);
        let g = rng.gen_range(0..127);
        let b = rng.gen_range(0..127);
        colored::customcolors::CustomColor::new(r, g, b)
    };

    for i in 0..s {
        for j in 0..s {
            let m = m[i][j];
            let cm = mask[i][j];
            let r = clusters.find(cm) as u32;

            let mut s = format!("({m}, {cm})");

            while s.len() < 10 {
                s.push(' ')
            }

            print!("{}", s.on_custom_color(random_color(r)))
        }
        println!();
    }

    // final pass
    let mut ans1 = 0;
    let total = regions.len();
    for (r, (a, p)) in regions {
        // println!("{r:<3}, a = {a:<3}, p = {p:<3}");
        ans1 += a as u64 * p as u64;
    }

    println!("total: {total}");

    println!("ans: {ans1}");

    ans1
}

fn parse(input: &str) -> Mat {
    input
        .lines()
        .map(|l| l.chars().map(|c| c as u32 - b'A' as u32 + 1).collect())
        .collect()
}

fn get_or0(m: &Mat, i: usize, j: usize, di: isize, dj: isize) -> u32 {
    let i = i as isize + di;
    let j = j as isize + dj;

    if i < 0 || j < 0 {
        return 0;
    }

    if i >= m.len() as isize || j >= m.len() as isize {
        return 0;
    }

    m[i as usize][j as usize]
}

fn printm(m: &Mat) {
    for r in m {
        for c in r {
            print!("{c:<6}");
        }
        println!();
    }
}

fn main() {
    // let tiny = parse(TINY);
    // solve1(&tiny);
    // println!();
    // let nested = parse(NESTED);
    // solve1(&nested);
    // println!();
    // let small = parse(SMALL);
    // solve1(&small);
    // println!();
    let input = parse(INPUT);
    solve1(&input);
    println!("toolow: 1412282");
}
