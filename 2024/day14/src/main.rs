use std::{thread::sleep, time::Duration};

use aoc::{Pos, PosI32};
use colored::{Colorize, CustomColor};

const INPUT: &str = include_str!("../input.txt");
const STEPS: u32 = 30000000;

// const W: u32 = 11;
// const H: u32 = 7;
const W: u32 = 101;
const H: u32 = 103;

const HW: u32 = W / 2;
const HH: u32 = H / 2;

fn parse(input: &str) -> impl Iterator<Item = (PosI32, PosI32)> + use<'_> {
    input.lines().filter_map(|l| {
        let (p, v) = l.split_once(' ')?;

        let p = p.split_once('=')?.1;
        let v = v.split_once('=')?.1;

        Some((p.parse().ok()?, v.parse().ok()?))
    })
}

fn hcolor(h: f32) -> CustomColor {
    // if min >= max {
    //     panic!("`min` must be less than `max`");
    // }

    // Normalize value to a range of 0.0 to 1.0
    let normalized = h as f32 / 50.0 as f32;

    // Convert normalized value to RGB heatmap
    let (r, g, b) = if normalized <= 0.5 {
        // Blue to Green gradient
        let ratio = normalized * 2.0;
        ((ratio * 200.0) as u8, 200, (100.0 * (1.0 - ratio)) as u8)
    } else {
        // Green to Red gradient
        let ratio = (normalized - 0.5) * 2.0;
        (255, (255.0 * (1.0 - ratio)) as u8, 0)
    };

    CustomColor::new(r, g, b)
}

fn solve1(input: &str) -> u32 {
    let mut pvs: Vec<(PosI32, PosI32)> = parse(INPUT).collect();
    let mut heatmap = vec![vec![0f32; W as usize]; H as usize];

    for i in 0..STEPS {
        for (mut p, ref v) in pvs.iter_mut() {
            p = p + *v;

            if p.0 < 0 {
                p.0 = W as i32 + p.0;
            }
            if p.1 < 0 {
                p.1 = H as i32 + p.1;
            }

            if p.0 >= W as i32 {
                p.0 %= W as i32;
            }
            if p.1 >= H as i32 {
                p.1 %= H as i32;
            }
        }

        for i in 0..H as usize {
            for j in 0..W as usize {
                heatmap[i][j] *= 0.9;

                for (p, _v) in pvs.iter() {
                    if p == &Pos(j as i32, i as i32) {
                        heatmap[i][j] += 1.0;
                    }
                }

            }
        }

        if i % 1000 == 0 {
            clearscreen::clear().unwrap();
            for i in 0..H as usize {
                for j in 0..W as usize {
                    print!("{}", "  ".on_custom_color(hcolor(heatmap[i][j])));
                }
                println!();
            }
            println!();
            // sleep(Duration::from_millis(1));

            println!("i: {i}");
        }
    }

    // for (mut p, v) in parse(INPUT) {
    //     // final_positions.push((p + v * STEPS) % Pos(W, H))
    //     for _ in 0..STEPS {
    //         p = p + v;

    //         if p.0 < 0 {
    //             p.0 = W as i32 + p.0;
    //         }
    //         if p.1 < 0 {
    //             p.1 = H as i32 + p.1;
    //         }

    //         if p.0 >= W as i32 {
    //             p.0 %= W as i32;
    //         }
    //         if p.1 >= H as i32 {
    //             p.1 %= H as i32;
    //         }

    //     }
    //     final_positions.push(p)
    // }

    // for i in 0..H as i32 {
    //     for j in 0..W as i32 {
    //         let mut n = 0;
    //         for fp in final_positions.iter() {
    //             if fp == &Pos(j, i) {
    //                 n += 1;
    //             }
    //         }
    //         if n > 0 {
    //             print!("{n}")
    //         } else {
    //             print!(".")
    //         }
    //     }
    //     println!()
    // }

    // dbg!(final_positions);

    let mut qs = [0; 4];

    for i in 0..H {
        for j in 0..W {
            if i == HH || j == HW {
                continue;
            }

            let q = if i < HH && j < HW {
                0
            } else if i < HH && j > HW {
                1
            } else if i > HH && j < HW {
                2
            } else {
                3
            };

            let mut n = 0;
            for (fp, _v) in pvs.iter() {
                if fp == &Pos(j as i32, i as i32) {
                    n += 1;
                }
            }
            qs[q] += n;
        }
        // println!()
    }

    // println!("{qs:?}");

    let mut r = 1;

    for q in qs {
        r *= q;
    }

    r
}

fn main() {
    println!("ans1: {}", solve1(INPUT));
    println!("exp: 12");
}
